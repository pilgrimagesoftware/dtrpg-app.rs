//! Rust SDK-backed implementation of [`LibraryService`].

use std::collections::HashMap;
use std::error::Error as StdError;

use dtrpg_sdk::{
    AuthTokenResponse, ClientError, Config, DriveThruRpgSdk, LibraryClient as SdkLibraryClient,
    LibraryItemsParams, OrderProductItem, OrderProductItemResponse, OrderProductListResponse,
    PublisherItem, SdkError,
};
use tokio::runtime::{Builder, Runtime};

use dtrpg_ui::{
    credentials::{CredentialStore, KeyringCredentialStore, keys},
    data::{
        enums::ItemStatus,
        library::LibraryItem,
    },
    services::{LibraryService, LibraryServiceError, LibraryServiceErrorKind},
};

const APPLICATION_KEY_ENV: &str = "DTRPG_APPLICATION_KEY";
const ACCESS_TOKEN_ENV: &str = "DTRPG_ACCESS_TOKEN";
const REFRESH_TOKEN_ENV: &str = "DTRPG_REFRESH_TOKEN";
const REFRESH_TOKEN_TTL_ENV: &str = "DTRPG_REFRESH_TOKEN_TTL";
const API_BASE_URL_ENV: &str = "DTRPG_API_BASE_URL";

const DEFAULT_COLOR: &str = "#2E3A45";
const BYTES_PER_MB: f64 = 1_048_576.0;

/// SDK operation boundary used by the Rust library service adapter.
pub trait SdkLibraryGateway: Send + Sync {
    /// Lists ordered products with optional pagination params.
    ///
    /// # Errors
    ///
    /// Returns [`LibraryServiceError`] on network or session failures.
    fn list_order_products(
        &self,
        params: LibraryItemsParams,
    ) -> Result<OrderProductListResponse, LibraryServiceError>;

    /// Loads an ordered product detail by its numeric id.
    ///
    /// # Errors
    ///
    /// Returns [`LibraryServiceError`] on network, session, or not-found failures.
    fn get_order_product(&self, id: u64) -> Result<OrderProductItemResponse, LibraryServiceError>;
}

/// Library service adapter backed by the Rust SDK.
pub struct RustSdkLibraryService {
    gateway: Box<dyn SdkLibraryGateway>,
}

impl RustSdkLibraryService {
    /// Creates a service from an SDK gateway implementation.
    pub fn new(gateway: Box<dyn SdkLibraryGateway>) -> Self {
        Self { gateway }
    }

    /// Creates the default service from environment-provided SDK configuration.
    ///
    /// Falls back to [`UnavailableSdkGateway`] when environment variables are absent.
    #[allow(dead_code)]
    pub fn from_environment() -> Self {
        match HttpSdkLibraryGateway::from_environment() {
            Ok(gateway) => Self::new(Box::new(gateway)),
            Err(error) => Self::new(Box::new(UnavailableSdkGateway::new(error))),
        }
    }

    /// Creates the service from credentials stored in the platform keyring.
    ///
    /// Keyring values take precedence; falls back to environment variables so
    /// development environments that set env vars still work. Falls back to
    /// [`UnavailableSdkGateway`] when neither source provides the required credentials.
    pub fn from_keyring() -> Self {
        match HttpSdkLibraryGateway::from_keyring() {
            Ok(gateway) => Self::new(Box::new(gateway)),
            Err(error) => Self::new(Box::new(UnavailableSdkGateway::new(error))),
        }
    }
}

impl LibraryService for RustSdkLibraryService {
    fn list_items(&self) -> Result<Vec<LibraryItem>, LibraryServiceError> {
        let mut all_items: Vec<OrderProductItem> = Vec::new();
        let mut all_included: Vec<PublisherItem> = Vec::new();
        let mut page: u32 = 1;

        loop {
            let params = LibraryItemsParams {
                page: Some(page),
                page_size: Some(100),
                get_checksum: Some(false),
                get_filters: Some(true),
                library: Some(true),
                archived: Some(false),
                updated_date_after: None,
            };

            let response = self.gateway.list_order_products(params)?;

            all_items.extend(response.data);
            if let Some(included) = response.included {
                for publisher in included {
                    let id = publisher.attributes.publisher_id;
                    if !all_included.iter().any(|p| p.attributes.publisher_id == id) {
                        all_included.push(publisher);
                    }
                }
            }

            if response.links.next.is_none() {
                break;
            }
            page += 1;
        }

        let publishers = publisher_lookup(&all_included);
        Ok(all_items
            .iter()
            .enumerate()
            .map(|(index, item)| map_order_product(item, &publishers, index as u32))
            .collect())
    }

    fn get_item(&self, id: u64) -> Result<LibraryItem, LibraryServiceError> {
        let response = self.gateway.get_order_product(id)?;
        Ok(map_order_product(&response.data, &HashMap::new(), 0))
    }
}

struct HttpSdkLibraryGateway {
    client: SdkLibraryClient,
    runtime: Runtime,
}

impl HttpSdkLibraryGateway {
    fn from_keyring() -> Result<Self, LibraryServiceError> {
        let application_key = KeyringCredentialStore::new(keys::SERVICE, keys::API_KEY)
            .load()
            .ok()
            .flatten()
            .map(|c| c.secret)
            .or_else(|| std::env::var(APPLICATION_KEY_ENV).ok())
            .ok_or_else(|| LibraryServiceError::new(
                LibraryServiceErrorKind::Session,
                "No API key found in keyring or environment. Sign in to continue.",
            ))?;

        let access_token = KeyringCredentialStore::new(keys::SERVICE, keys::ACCESS_TOKEN)
            .load()
            .ok()
            .flatten()
            .map(|c| c.secret)
            .or_else(|| std::env::var(ACCESS_TOKEN_ENV).ok())
            .ok_or_else(|| LibraryServiceError::new(
                LibraryServiceErrorKind::Session,
                "No access token found. Full authentication is pending a future update.",
            ))?;

        let refresh_token = KeyringCredentialStore::new(keys::SERVICE, keys::REFRESH_TOKEN)
            .load()
            .ok()
            .flatten()
            .map(|c| c.secret)
            .unwrap_or_else(|| std::env::var(REFRESH_TOKEN_ENV).unwrap_or_default());

        let refresh_token_ttl = std::env::var(REFRESH_TOKEN_TTL_ENV)
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(u64::MAX);

        Self::build(application_key, access_token, refresh_token, refresh_token_ttl)
    }

    #[allow(dead_code)]
    fn from_environment() -> Result<Self, LibraryServiceError> {
        let application_key = std::env::var(APPLICATION_KEY_ENV).map_err(|_| {
            LibraryServiceError::new(
                LibraryServiceErrorKind::Network,
                format!("{APPLICATION_KEY_ENV} is required to load SDK-backed library data."),
            )
        })?;
        let access_token = std::env::var(ACCESS_TOKEN_ENV).map_err(|_| {
            LibraryServiceError::new(
                LibraryServiceErrorKind::Session,
                format!("{ACCESS_TOKEN_ENV} is required to load SDK-backed library data."),
            )
        })?;
        let refresh_token = std::env::var(REFRESH_TOKEN_ENV).unwrap_or_default();
        let refresh_token_ttl = std::env::var(REFRESH_TOKEN_TTL_ENV)
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(u64::MAX);

        Self::build(application_key, access_token, refresh_token, refresh_token_ttl)
    }

    fn build(
        application_key: String,
        access_token: String,
        refresh_token: String,
        refresh_token_ttl: u64,
    ) -> Result<Self, LibraryServiceError> {
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
            .map_err(|error| {
                LibraryServiceError::new(
                    LibraryServiceErrorKind::Network,
                    format!("Unable to start Rust SDK runtime: {error}"),
                )
            })?;

        Ok(Self { client, runtime })
    }
}

impl SdkLibraryGateway for HttpSdkLibraryGateway {
    fn list_order_products(
        &self,
        params: LibraryItemsParams,
    ) -> Result<OrderProductListResponse, LibraryServiceError> {
        self.runtime
            .block_on(self.client.list_order_products(params))
            .map_err(map_client_error)
    }

    fn get_order_product(&self, id: u64) -> Result<OrderProductItemResponse, LibraryServiceError> {
        self.runtime
            .block_on(self.client.get_order_product(id))
            .map_err(map_client_error)
    }
}

struct UnavailableSdkGateway {
    error: LibraryServiceError,
}

impl UnavailableSdkGateway {
    fn new(error: LibraryServiceError) -> Self {
        Self { error }
    }
}

impl SdkLibraryGateway for UnavailableSdkGateway {
    fn list_order_products(
        &self,
        _params: LibraryItemsParams,
    ) -> Result<OrderProductListResponse, LibraryServiceError> {
        Err(self.error.clone())
    }

    fn get_order_product(&self, _id: u64) -> Result<OrderProductItemResponse, LibraryServiceError> {
        Err(self.error.clone())
    }
}

fn publisher_lookup(included: &[PublisherItem]) -> HashMap<u64, String> {
    included
        .iter()
        .map(|publisher| {
            (
                publisher.attributes.publisher_id,
                publisher.attributes.name.clone(),
            )
        })
        .collect()
}

fn map_order_product(
    item: &OrderProductItem,
    publishers: &HashMap<u64, String>,
    order: u32,
) -> LibraryItem {
    let attributes = &item.attributes;

    let numeric_id = attributes
        .order_product_id
        .max(item.id.parse::<u64>().unwrap_or_default())
        .max(attributes.product_id);

    let publisher = publishers
        .get(&attributes.royalty_publisher_id)
        .cloned()
        .unwrap_or_else(|| format!("Publisher {}", attributes.royalty_publisher_id));

    let kind = attributes
        .filters
        .as_ref()
        .and_then(|filters| filters.iter().find(|f| f.parent_filter_id == 0))
        .map(|f| if f.parent_name.is_empty() { f.name.clone() } else { f.parent_name.clone() })
        .unwrap_or_else(|| "Library item".to_string());

    let mut format_parts: Vec<String> = attributes
        .files
        .iter()
        .map(|f| f.title.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    format_parts.sort();
    let format = if format_parts.is_empty() {
        "PDF".to_string()
    } else {
        format_parts.join(" + ")
    };

    let size_mb = attributes
        .files
        .iter()
        .map(|f| f.size as f64)
        .sum::<f64>()
        / BYTES_PER_MB;

    let year = attributes
        .file_last_modified
        .as_deref()
        .or(attributes.date_purchased.as_deref())
        .and_then(|date| date.get(..4))
        .and_then(|y| y.parse::<u32>().ok())
        .unwrap_or(0);

    let desc = {
        let mut parts = Vec::new();
        if let Some(date) = &attributes.date_purchased {
            parts.push(format!("Purchased {date}"));
        }
        if let Some(date) = &attributes.file_last_modified {
            parts.push(format!("Updated {date}"));
        }
        parts.join(". ")
    };

    LibraryItem {
        id: item.id.as_str().into(),
        numeric_id,
        title: attributes.name.as_str().into(),
        publisher: publisher.as_str().into(),
        line: "".into(),
        kind: kind.as_str().into(),
        format: format.as_str().into(),
        pages: 0,
        size_mb,
        year,
        added_order: order,
        status: ItemStatus::Cloud,
        color: DEFAULT_COLOR.into(),
        desc: desc.as_str().into(),
        cover_url: None,
    }
}

fn map_client_error(error: ClientError) -> LibraryServiceError {
    match error {
        ClientError::Sdk(error) => map_sdk_error(error),
        ClientError::Http(error) => {
            let status = error.status().map(|s| s.as_u16());
            let kind = match status {
                Some(401 | 403) => LibraryServiceErrorKind::Session,
                _ => LibraryServiceErrorKind::Network,
            };

            let mut msg = String::from("Rust SDK library request failed");
            if let Some(url) = error.url() {
                msg.push_str(&format!(" [{url}]"));
            }
            if let Some(code) = status {
                msg.push_str(&format!(" (HTTP {code})"));
            }
            msg.push_str(&format!(": {error}"));

            // Walk the source chain so serde decode errors include the field/line detail.
            let mut source: Option<&dyn StdError> = StdError::source(&error);
            while let Some(cause) = source {
                msg.push_str(&format!(": {cause}"));
                source = cause.source();
            }

            LibraryServiceError::new(kind, msg)
        }
    }
}

fn map_sdk_error(error: SdkError) -> LibraryServiceError {
    let kind = match error {
        SdkError::Unauthenticated | SdkError::AuthSession(_) => LibraryServiceErrorKind::Session,
        SdkError::Unconfigured => LibraryServiceErrorKind::Network,
    };
    LibraryServiceError::new(kind, format!("Rust SDK is not ready: {error}"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};

    use dtrpg_sdk::{
        FileChecksum, OrderProductAttributes, OrderProductFile, PaginationLinks, PaginationMeta,
        PublisherAttributes,
    };

    use super::*;

    struct FakeSdkGateway {
        list_result: Result<OrderProductListResponse, LibraryServiceError>,
        detail_result: Result<OrderProductItemResponse, LibraryServiceError>,
    }

    impl FakeSdkGateway {
        fn seeded() -> Self {
            let item = order_product_item(42, "A Better Dungeon");
            Self {
                list_result: Ok(OrderProductListResponse {
                    links: pagination_links(None),
                    meta: PaginationMeta {
                        items_per_page: 100,
                        current_page: 1,
                    },
                    data: vec![item.clone()],
                    included: Some(vec![PublisherItem {
                        id: "7".to_string(),
                        resource_type: "publisher".to_string(),
                        attributes: PublisherAttributes {
                            name: "Lantern Press".to_string(),
                            publisher_id: 7,
                            slug: "lantern-press".to_string(),
                        },
                    }]),
                }),
                detail_result: Ok(OrderProductItemResponse { data: item }),
            }
        }

        fn session_error() -> Self {
            let error = LibraryServiceError::new(
                LibraryServiceErrorKind::Session,
                "SDK session expired in test.",
            );
            Self {
                list_result: Err(error.clone()),
                detail_result: Err(error),
            }
        }
    }

    impl SdkLibraryGateway for FakeSdkGateway {
        fn list_order_products(
            &self,
            _params: LibraryItemsParams,
        ) -> Result<OrderProductListResponse, LibraryServiceError> {
            self.list_result.clone()
        }

        fn get_order_product(
            &self,
            _id: u64,
        ) -> Result<OrderProductItemResponse, LibraryServiceError> {
            self.detail_result.clone()
        }
    }

    /// Returns pages in order: first call gets page 1 with a `next` link,
    /// second call gets page 2 with no `next` link.
    struct TwoPageGateway {
        call_count: AtomicU32,
    }

    impl SdkLibraryGateway for TwoPageGateway {
        fn list_order_products(
            &self,
            _params: LibraryItemsParams,
        ) -> Result<OrderProductListResponse, LibraryServiceError> {
            let call = self.call_count.fetch_add(1, Ordering::Relaxed);

            if call == 0 {
                Ok(OrderProductListResponse {
                    links: pagination_links(Some("page=2".to_string())),
                    meta: PaginationMeta { items_per_page: 1, current_page: 1 },
                    data: vec![order_product_item(42, "Item Page One")],
                    included: None,
                })
            } else {
                Ok(OrderProductListResponse {
                    links: pagination_links(None),
                    meta: PaginationMeta { items_per_page: 1, current_page: 2 },
                    data: vec![order_product_item(99, "Item Page Two")],
                    included: None,
                })
            }
        }

        fn get_order_product(
            &self,
            _id: u64,
        ) -> Result<OrderProductItemResponse, LibraryServiceError> {
            Err(LibraryServiceError::new(LibraryServiceErrorKind::NotFound, "not used"))
        }
    }

    #[test]
    fn sdk_service_maps_order_products_to_library_items() {
        let service = RustSdkLibraryService::new(Box::new(FakeSdkGateway::seeded()));

        let items = service.list_items().expect("list items");

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].numeric_id, 42);
        assert_eq!(items[0].title.as_ref(), "A Better Dungeon");
        assert_eq!(items[0].publisher.as_ref(), "Lantern Press");
        assert_eq!(items[0].kind.as_ref(), "Adventure");
    }

    #[test]
    fn sdk_service_preserves_session_error_classification() {
        let service = RustSdkLibraryService::new(Box::new(FakeSdkGateway::session_error()));

        let error = service.list_items().expect_err("session error");

        assert_eq!(error.kind, LibraryServiceErrorKind::Session);
    }

    fn order_product_item(id: u64, name: &str) -> OrderProductItem {
        OrderProductItem {
            id: id.to_string(),
            resource_type: "order_product".to_string(),
            attributes: OrderProductAttributes {
                order_id: 900,
                product_id: id,
                royalty_publisher_id: 7,
                isbn: None,
                name: name.to_string(),
                date_purchased: Some("2026-01-01".to_string()),
                filesize: Some(1024),
                final_price: 12.5,
                quantity: 1,
                bundle_id: 0,
                archived: 0,
                add_on_info: None,
                order_product_id: id,
                customer_id: 123,
                file_last_modified: Some("2026-01-02".to_string()),
                file_last_downloaded: None,
                files: vec![OrderProductFile {
                    index: 0,
                    order_product_download_id: 1234,
                    title: "PDF".to_string(),
                    filename: "better-dungeon.pdf".to_string(),
                    size: 1_048_576,
                    size_mb: "1.0".to_string(),
                    checksums: vec![FileChecksum {
                        checksum: "abc123".to_string(),
                        checksum_date: "2026-01-02".to_string(),
                    }],
                }],
                filters: Some(vec![dtrpg_sdk::OrderProductFilter {
                    filter_id: 1,
                    parent_filter_id: 0,
                    name: "Dungeon".to_string(),
                    parent_name: "Adventure".to_string(),
                }]),
                history: None,
                attributes: None,
            },
        }
    }

    fn pagination_links(next: Option<String>) -> PaginationLinks {
        PaginationLinks {
            self_: "self".to_string(),
            first: None,
            last: None,
            prev: None,
            next,
        }
    }

    #[test]
    fn sdk_service_fetches_all_pages_via_pagination() {
        let gateway = TwoPageGateway { call_count: AtomicU32::new(0) };
        let service = RustSdkLibraryService::new(Box::new(gateway));

        let items = service.list_items().expect("list items");

        assert_eq!(items.len(), 2, "should have items from both pages");
        assert!(
            items.iter().any(|i| i.numeric_id == 42),
            "page 1 item should be present"
        );
        assert!(
            items.iter().any(|i| i.numeric_id == 99),
            "page 2 item should be present"
        );
    }
}
