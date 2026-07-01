//! Rust SDK-backed implementation of [`CollectionsService`].

use std::sync::Arc;

use dtrpg_sdk::{
    AuthTokenResponse, ClientError, Config, DriveThruRpgSdk, LibraryClient as SdkLibraryClient,
    PageParams, ProductListCollectionResponse, ProductListItem, ProductListItemsResponse, SdkError,
};
use tokio::runtime::{Builder, Runtime};

use crate::constants::{
    ACCESS_TOKEN_ENV, API_BASE_URL_ENV, APPLICATION_KEY_ENV, REFRESH_TOKEN_ENV,
    REFRESH_TOKEN_TTL_ENV,
};
use dtrpg_ui::{
    credentials::{CredentialStore, KeyringCredentialStore},
    data::collection::CollectionEntry,
    data::constants::{KEYRING_API_KEY, KEYRING_SERVICE},
    services::{
        LoginTokens,
        collections::{CollectionsService, CollectionsServiceError, CollectionsServiceErrorKind},
    },
};

// ── Gateway trait ─────────────────────────────────────────────────────────────

/// SDK operation boundary used by the Rust collections service adapter.
pub trait SdkCollectionsGateway: Send + Sync {
    /// Lists the user's DTRPG product lists, paginating through all pages.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionsServiceError`] on network or session failures.
    fn list_product_lists(
        &self,
        page: u32,
    ) -> Result<ProductListCollectionResponse, CollectionsServiceError>;

    /// Lists the items within a specific product list, paginating through all pages.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionsServiceError`] on network or session failures.
    fn list_product_list_items(
        &self,
        product_list_id: u64,
        page: u32,
    ) -> Result<ProductListItemsResponse, CollectionsServiceError>;

    /// Creates a new product list with the given name and returns it.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionsServiceError`] on network or session failures.
    fn create_product_list(&self, name: &str) -> Result<ProductListItem, CollectionsServiceError>;

    /// Deletes the product list with the given id.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionsServiceError`] on network or session failures.
    fn delete_product_list(&self, id: u64) -> Result<(), CollectionsServiceError>;
}

// ── Service implementation ────────────────────────────────────────────────────

/// Collections service adapter backed by the Rust SDK.
pub struct RustSdkCollectionsService {
    gateway: Box<dyn SdkCollectionsGateway>,
}

impl RustSdkCollectionsService {
    /// Creates a service from an SDK gateway implementation.
    pub fn new(gateway: Box<dyn SdkCollectionsGateway>) -> Self {
        Self { gateway }
    }

    /// Creates an unauthenticated service that returns a session error on all calls.
    pub fn unauthenticated() -> Self {
        Self::new(Box::new(UnavailableCollectionsGateway::new(
            CollectionsServiceError::new(
                CollectionsServiceErrorKind::Session,
                "Not signed in. Open Settings > Account to sign in.",
            ),
        )))
    }

    /// Creates the service using in-memory `tokens` obtained at startup or login.
    ///
    /// Reads only the API key from the platform keyring. Falls back to
    /// [`UnavailableCollectionsGateway`] when the API key cannot be found.
    pub fn from_keyring_with_tokens(tokens: LoginTokens) -> Self {
        match HttpSdkCollectionsGateway::from_keyring_with_tokens(tokens) {
            Ok(gateway) => Self::new(Box::new(gateway)),
            Err(error) => Self::new(Box::new(UnavailableCollectionsGateway::new(error))),
        }
    }
}

impl CollectionsService for RustSdkCollectionsService {
    fn list_collections(&self) -> Result<Vec<CollectionEntry>, CollectionsServiceError> {
        // Fetch all product list pages.
        let mut all_lists = Vec::new();
        let mut page: u32 = 1;
        loop {
            let response = self.gateway.list_product_lists(page)?;
            all_lists.extend(response.data);
            if response.links.next.is_none() {
                break;
            }
            page += 1;
        }

        // For each list, fetch all member IDs.
        let mut entries = Vec::with_capacity(all_lists.len());
        for list in all_lists {
            let id = list.attributes.product_list_id;
            let name: Arc<str> = list.attributes.name.as_str().into();

            let mut member_ids: Vec<u64> = Vec::new();
            let mut items_page: u32 = 1;
            loop {
                let items_resp = match self.gateway.list_product_list_items(id, items_page) {
                    Ok(r) => r,
                    Err(e) => {
                        tracing::warn!(
                            collection_id = id,
                            error = %e,
                            "failed to fetch collection items, skipping"
                        );
                        break;
                    }
                };

                for item in &items_resp.data {
                    // Try orderProductId first (flat or JSON:API-style), then productId.
                    // The product_list_items endpoint returns productId in attributes;
                    // both IDs are stored so the sidebar filter can match either.
                    let extracted = item
                        .get("orderProductId")
                        .and_then(|v| v.as_u64())
                        .or_else(|| {
                            item.get("attributes")
                                .and_then(|a| a.get("orderProductId"))
                                .and_then(|v| v.as_u64())
                        })
                        .or_else(|| {
                            item.get("attributes")
                                .and_then(|a| a.get("productId"))
                                .and_then(|v| v.as_u64())
                        });

                    if let Some(id_value) = extracted {
                        member_ids.push(id_value);
                    } else {
                        tracing::warn!(
                            collection_id = id,
                            item = ?item,
                            "skipping list item: no orderProductId or productId found"
                        );
                    }
                }

                if items_resp.links.next.is_none() {
                    break;
                }
                items_page += 1;
            }

            entries.push(CollectionEntry {
                id,
                name,
                member_ids: Arc::from(member_ids.as_slice()),
            });
        }

        Ok(entries)
    }

    fn create_collection(&self, name: &str) -> Result<CollectionEntry, CollectionsServiceError> {
        let item = self.gateway.create_product_list(name.trim())?;
        Ok(CollectionEntry {
            id: item.attributes.product_list_id,
            name: Arc::from(item.attributes.name.as_str()),
            member_ids: Arc::from(&[][..]),
        })
    }

    fn delete_collection(&self, id: u64) -> Result<(), CollectionsServiceError> {
        self.gateway.delete_product_list(id)
    }
}

// ── HTTP gateway ──────────────────────────────────────────────────────────────

struct HttpSdkCollectionsGateway {
    client: SdkLibraryClient,
    runtime: Runtime,
}

impl HttpSdkCollectionsGateway {
    fn from_keyring_with_tokens(tokens: LoginTokens) -> Result<Self, CollectionsServiceError> {
        let application_key = KeyringCredentialStore::new(KEYRING_SERVICE, KEYRING_API_KEY)
            .load()
            .ok()
            .flatten()
            .map(|c| c.secret)
            .or_else(|| std::env::var(APPLICATION_KEY_ENV).ok())
            .ok_or_else(|| {
                CollectionsServiceError::new(
                    CollectionsServiceErrorKind::Session,
                    "No API key found in keyring or environment. Sign in to continue.",
                )
            })?;

        Self::build(
            application_key,
            tokens.access_token,
            tokens.refresh_token,
            tokens.refresh_token_ttl,
        )
    }

    #[allow(dead_code)]
    fn from_environment() -> Result<Self, CollectionsServiceError> {
        let application_key = std::env::var(APPLICATION_KEY_ENV).map_err(|_| {
            CollectionsServiceError::new(
                CollectionsServiceErrorKind::Network,
                format!("{APPLICATION_KEY_ENV} is required to load SDK-backed collections data."),
            )
        })?;
        let access_token = std::env::var(ACCESS_TOKEN_ENV).map_err(|_| {
            CollectionsServiceError::new(
                CollectionsServiceErrorKind::Session,
                format!("{ACCESS_TOKEN_ENV} is required to load SDK-backed collections data."),
            )
        })?;
        let refresh_token = std::env::var(REFRESH_TOKEN_ENV).unwrap_or_default();
        let refresh_token_ttl = std::env::var(REFRESH_TOKEN_TTL_ENV)
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(u64::MAX);

        Self::build(
            application_key,
            access_token,
            refresh_token,
            refresh_token_ttl,
        )
    }

    fn build(
        application_key: String,
        access_token: String,
        refresh_token: String,
        refresh_token_ttl: u64,
    ) -> Result<Self, CollectionsServiceError> {
        let config = match std::env::var(API_BASE_URL_ENV) {
            Ok(base_url) => Config::with_base_url(application_key, base_url),
            Err(_) => Config::new(application_key),
        };

        let mut sdk = DriveThruRpgSdk::with_config(config);
        sdk.apply_auth_response(AuthTokenResponse::new(
            access_token,
            refresh_token,
            refresh_token_ttl,
        ))
        .map_err(map_sdk_error)?;

        let client = sdk.library_client().map_err(map_sdk_error)?;
        let runtime = Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| {
                CollectionsServiceError::new(
                    CollectionsServiceErrorKind::Network,
                    format!("Unable to start Rust SDK runtime: {e}"),
                )
            })?;

        Ok(Self { client, runtime })
    }
}

impl SdkCollectionsGateway for HttpSdkCollectionsGateway {
    fn list_product_lists(
        &self,
        page: u32,
    ) -> Result<ProductListCollectionResponse, CollectionsServiceError> {
        self.runtime
            .block_on(self.client.list_product_lists(PageParams {
                page: Some(page),
                page_size: None,
            }))
            .map_err(map_client_error)
    }

    fn list_product_list_items(
        &self,
        product_list_id: u64,
        page: u32,
    ) -> Result<ProductListItemsResponse, CollectionsServiceError> {
        self.runtime
            .block_on(self.client.list_product_list_items(
                product_list_id,
                PageParams {
                    page: Some(page),
                    page_size: None,
                },
            ))
            .map_err(map_client_error)
    }

    fn create_product_list(&self, name: &str) -> Result<ProductListItem, CollectionsServiceError> {
        self.runtime
            .block_on(self.client.create_product_list(name))
            .map_err(map_client_error)
    }

    fn delete_product_list(&self, id: u64) -> Result<(), CollectionsServiceError> {
        self.runtime
            .block_on(self.client.delete_product_list(id))
            .map_err(map_client_error)
    }
}

// ── Unavailable gateway ───────────────────────────────────────────────────────

/// A gateway that returns a stored error on every call.
///
/// Used when SDK initialization fails (missing API key, invalid credentials)
/// so the app starts without crashing. The library window still opens; the
/// Collections section simply remains absent from the sidebar.
struct UnavailableCollectionsGateway {
    error: CollectionsServiceError,
}

impl UnavailableCollectionsGateway {
    fn new(error: CollectionsServiceError) -> Self {
        Self { error }
    }
}

impl SdkCollectionsGateway for UnavailableCollectionsGateway {
    fn list_product_lists(
        &self,
        _page: u32,
    ) -> Result<ProductListCollectionResponse, CollectionsServiceError> {
        Err(self.error.clone())
    }

    fn list_product_list_items(
        &self,
        _product_list_id: u64,
        _page: u32,
    ) -> Result<ProductListItemsResponse, CollectionsServiceError> {
        Err(self.error.clone())
    }

    fn create_product_list(&self, _name: &str) -> Result<ProductListItem, CollectionsServiceError> {
        Err(self.error.clone())
    }

    fn delete_product_list(&self, _id: u64) -> Result<(), CollectionsServiceError> {
        Err(self.error.clone())
    }
}

// ── Error mapping ─────────────────────────────────────────────────────────────

fn map_client_error(error: ClientError) -> CollectionsServiceError {
    match error {
        ClientError::Sdk(e) => map_sdk_error(e),
        ClientError::DecodeFailed {
            url, status, cause, ..
        } => {
            let kind = match status {
                401 | 403 => CollectionsServiceErrorKind::Session,
                _ => CollectionsServiceErrorKind::Network,
            };
            CollectionsServiceError::new(
                kind,
                format!("Response from {url} (HTTP {status}) could not be decoded: {cause}"),
            )
        }
        ClientError::Http(error) => {
            let status = error.status().map(|s| s.as_u16());
            let kind = match status {
                Some(401) | Some(403) => CollectionsServiceErrorKind::Session,
                _ => CollectionsServiceErrorKind::Network,
            };
            let mut msg = String::from("Rust SDK collections request failed");
            if let Some(url) = error.url() {
                msg.push_str(&format!(" [{url}]"));
            }
            if let Some(code) = status {
                msg.push_str(&format!(" (HTTP {code})"));
            }
            msg.push_str(&format!(": {error}"));
            CollectionsServiceError::new(kind, msg)
        }
    }
}

fn map_sdk_error(error: SdkError) -> CollectionsServiceError {
    let kind = match error {
        SdkError::Unauthenticated | SdkError::AuthSession(_) => {
            CollectionsServiceErrorKind::Session
        }
        SdkError::Unconfigured => CollectionsServiceErrorKind::Network,
    };
    CollectionsServiceError::new(kind, format!("Rust SDK is not ready: {error}"))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use dtrpg_sdk::{PaginationLinks, PaginationMeta, ProductListAttributes, ProductListItem};
    use serde_json::json;

    use super::*;

    fn pagination_links(next: Option<&str>) -> PaginationLinks {
        PaginationLinks {
            self_: "self".to_string(),
            first: None,
            last: None,
            prev: None,
            next: next.map(String::from),
        }
    }

    fn product_list_item(id: u64, name: &str) -> ProductListItem {
        ProductListItem {
            id: id.to_string(),
            resource_type: "product_list".to_string(),
            attributes: ProductListAttributes {
                customer_id: 100,
                name: name.to_string(),
                date_created: "2026-01-01".to_string(),
                product_list_id: id,
                slug: name.to_lowercase().replace(' ', "-"),
                item_count: 2,
            },
        }
    }

    struct FakeCollectionsGateway {
        lists: Result<Vec<ProductListItem>, CollectionsServiceError>,
        items: Result<Vec<serde_json::Value>, CollectionsServiceError>,
        create_result: Result<ProductListItem, CollectionsServiceError>,
    }

    impl FakeCollectionsGateway {
        fn seeded() -> Self {
            Self {
                lists: Ok(vec![product_list_item(7, "Favorites")]),
                items: Ok(vec![
                    json!({ "orderProductId": 42 }),
                    json!({ "orderProductId": 99 }),
                ]),
                create_result: Ok(product_list_item(8, "New List")),
            }
        }

        fn session_error() -> Self {
            let err = CollectionsServiceError::new(
                CollectionsServiceErrorKind::Session,
                "fake session error",
            );
            Self {
                lists: Err(err.clone()),
                items: Err(err.clone()),
                create_result: Err(err),
            }
        }

        fn items_with_missing_field() -> Self {
            Self {
                lists: Ok(vec![product_list_item(7, "Favorites")]),
                items: Ok(vec![
                    json!({ "orderProductId": 42 }),
                    json!({ "someOtherField": "abc" }),
                    json!({ "orderProductId": 99 }),
                ]),
                create_result: Ok(product_list_item(8, "New List")),
            }
        }
    }

    impl SdkCollectionsGateway for FakeCollectionsGateway {
        fn list_product_lists(
            &self,
            _page: u32,
        ) -> Result<ProductListCollectionResponse, CollectionsServiceError> {
            self.lists
                .clone()
                .map(|data| ProductListCollectionResponse {
                    links: pagination_links(None),
                    meta: PaginationMeta {
                        items_per_page: 100,
                        current_page: 1,
                    },
                    data,
                })
        }

        fn list_product_list_items(
            &self,
            _product_list_id: u64,
            _page: u32,
        ) -> Result<ProductListItemsResponse, CollectionsServiceError> {
            self.items.clone().map(|data| ProductListItemsResponse {
                links: pagination_links(None),
                meta: PaginationMeta {
                    items_per_page: 100,
                    current_page: 1,
                },
                data,
            })
        }

        fn create_product_list(
            &self,
            _name: &str,
        ) -> Result<ProductListItem, CollectionsServiceError> {
            self.create_result.clone()
        }

        fn delete_product_list(&self, _id: u64) -> Result<(), CollectionsServiceError> {
            self.lists.as_ref().map(|_| ()).map_err(Clone::clone)
        }
    }

    #[test]
    fn seeded_data_produces_correct_collection_entries() {
        let service = RustSdkCollectionsService::new(Box::new(FakeCollectionsGateway::seeded()));
        let collections = service.list_collections().expect("list collections");

        assert_eq!(collections.len(), 1);
        let entry = &collections[0];
        assert_eq!(entry.id, 7);
        assert_eq!(entry.name.as_ref(), "Favorites");
        assert_eq!(entry.member_ids.as_ref(), &[42u64, 99u64]);
    }

    #[test]
    fn session_error_propagates() {
        let service =
            RustSdkCollectionsService::new(Box::new(FakeCollectionsGateway::session_error()));
        let err = service.list_collections().expect_err("session error");
        assert_eq!(err.kind, CollectionsServiceErrorKind::Session);
    }

    #[test]
    fn items_with_missing_order_product_id_are_skipped() {
        let service = RustSdkCollectionsService::new(Box::new(
            FakeCollectionsGateway::items_with_missing_field(),
        ));
        let collections = service.list_collections().expect("list collections");
        assert_eq!(collections.len(), 1);
        let member_ids = &collections[0].member_ids;
        assert_eq!(member_ids.as_ref(), &[42u64, 99u64]);
    }

    #[test]
    fn create_collection_returns_correct_entry() {
        let service = RustSdkCollectionsService::new(Box::new(FakeCollectionsGateway::seeded()));
        let entry = service
            .create_collection("New List")
            .expect("create collection");

        assert_eq!(entry.id, 8);
        assert_eq!(entry.name.as_ref(), "New List");
        assert!(entry.member_ids.is_empty());
    }

    #[test]
    fn create_collection_propagates_session_error() {
        let service =
            RustSdkCollectionsService::new(Box::new(FakeCollectionsGateway::session_error()));
        let err = service
            .create_collection("Anything")
            .expect_err("session error");
        assert_eq!(err.kind, CollectionsServiceErrorKind::Session);
    }
}
