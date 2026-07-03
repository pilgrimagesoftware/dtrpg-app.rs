//! Rust SDK-backed implementation of [`LibraryService`].

use std::collections::{HashMap, HashSet};
use std::error::Error as StdError;

use dtrpg_sdk::{
    AuthTokenResponse, ClientError, Config, DriveThruRpgSdk, IncludedItem,
    LibraryClient as SdkLibraryClient, LibraryItemsParams, OrderProductInfo, OrderProductItem,
    OrderProductItemResponse, OrderProductListResponse, PaginationLinks, SdkError,
};
use tokio::runtime::{Builder, Runtime};

use dtrpg_ui::{
    credentials::{CredentialStore, KeyringCredentialStore},
    data::constants::{KEYRING_API_KEY, KEYRING_SERVICE},
    data::{enums::ItemStatus, library::LibraryItem},
    services::{LibraryService, LibraryServiceError, LibraryServiceErrorKind},
};

use crate::constants::{
    ACCESS_TOKEN_ENV, API_BASE_URL_ENV, APPLICATION_KEY_ENV, BYTES_PER_MB, DEFAULT_COLOR,
    DTRPG_IMAGES_BASE_URL, REFRESH_TOKEN_ENV, REFRESH_TOKEN_TTL_ENV,
};

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

    /// Creates an unauthenticated service that returns a "not signed in" error on all calls.
    pub fn unauthenticated() -> Self {
        Self::new(Box::new(UnavailableSdkGateway::new(
            LibraryServiceError::new(
                LibraryServiceErrorKind::Session,
                "Not signed in. Open Settings > Account to sign in.",
            ),
        )))
    }

    /// Creates the service using in-memory `tokens` obtained at startup or login.
    ///
    /// Reads only the API key from the platform keyring; tokens are never persisted
    /// to the keychain. Falls back to [`UnavailableSdkGateway`] when the API key
    /// cannot be found.
    pub fn from_keyring_with_tokens(tokens: dtrpg_ui::services::LoginTokens) -> Self {
        match HttpSdkLibraryGateway::from_keyring_with_tokens(tokens) {
            Ok(gateway) => Self::new(Box::new(gateway)),
            Err(error) => Self::new(Box::new(UnavailableSdkGateway::new(error))),
        }
    }
}

impl LibraryService for RustSdkLibraryService {
    fn list_items(&self) -> Result<Vec<LibraryItem>, LibraryServiceError> {
        let mut all_items: Vec<LibraryItem> = Vec::new();
        self.list_items_paged(&mut |page_items| all_items.extend(page_items), None)?;
        Ok(all_items)
    }

    fn list_items_paged(
        &self,
        on_page: &mut dyn FnMut(Vec<LibraryItem>),
        mut on_total: Option<&mut dyn FnMut(usize)>,
    ) -> Result<(), LibraryServiceError> {
        let mut all_included: Vec<IncludedItem> = Vec::new();
        let mut page: u32 = 1;
        let mut global_index: u32 = 0;
        let mut total_reported = false;
        let page_size: u32 = 100;

        loop {
            let params = LibraryItemsParams {
                page: Some(page),
                page_size: Some(page_size),
                get_checksum: Some(false),
                get_filters: Some(true),
                library: Some(true),
                archived: Some(false),
                updated_date_after: None,
            };

            let response = self.gateway.list_order_products(params)?;

            // Derive estimated total from `links.last` on the first page response.
            if !total_reported {
                if let (Some(cb), Some(last_page)) = (
                    on_total.as_deref_mut(),
                    last_page_from_links(&response.links),
                ) {
                    let estimated = (last_page as usize).saturating_mul(page_size as usize);
                    cb(estimated);
                }
                total_reported = true;
            }

            // `Product` resources are only needed to resolve this page's items — rebuilt
            // fresh per page rather than accumulated, since each ordered product's
            // relationship points at its own distinct `Product` resource.
            let products = response
                .included
                .as_deref()
                .map(product_lookup)
                .unwrap_or_default();

            if let Some(included) = &response.included {
                for entry in included {
                    if entry.resource_type == "Publisher"
                        && !all_included.iter().any(|p| p.id == entry.id)
                    {
                        all_included.push(entry.clone());
                    }
                }
            }

            let has_next = response.links.next.is_some();
            let publishers = publisher_lookup(&all_included);
            let page_items: Vec<LibraryItem> = response
                .data
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    map_order_product(item, &publishers, &products, global_index + i as u32)
                })
                .collect();

            global_index += page_items.len() as u32;
            on_page(page_items);

            if !has_next {
                break;
            }
            page += 1;
        }

        Ok(())
    }

    fn get_item(&self, id: u64) -> Result<LibraryItem, LibraryServiceError> {
        let response = self.gateway.get_order_product(id)?;
        // The single-item detail endpoint has no `included` sideload array to resolve
        // `relationships` against; falls back to `attributes.product` if ever embedded.
        Ok(map_order_product(
            &response.data,
            &HashMap::new(),
            &HashMap::new(),
            0,
        ))
    }

    fn count_items(&self) -> Option<Result<usize, LibraryServiceError>> {
        // Request a single item per page: with `pageSize=1`, the last page number
        // reported in `links.last` is numerically equal to the total item count,
        // so this is a cheap way to detect remote changes without fetching all
        // pages. Falls back to the single returned page's length when there is no
        // `last` link (i.e. the whole library fits on one page of size 1: 0 or 1
        // items).
        let params = LibraryItemsParams {
            page: Some(1),
            page_size: Some(1),
            get_checksum: Some(false),
            get_filters: Some(false),
            library: Some(true),
            archived: Some(false),
            updated_date_after: None,
        };

        Some(self.gateway.list_order_products(params).map(|response| {
            last_page_from_links(&response.links)
                .map(|last_page| last_page as usize)
                .unwrap_or(response.data.len())
        }))
    }
}

struct HttpSdkLibraryGateway {
    client: SdkLibraryClient,
    runtime: Runtime,
}

impl HttpSdkLibraryGateway {
    /// Reads only the API key from the keyring and uses the provided in-memory tokens.
    fn from_keyring_with_tokens(
        tokens: dtrpg_ui::services::LoginTokens,
    ) -> Result<Self, LibraryServiceError> {
        let application_key = KeyringCredentialStore::new(KEYRING_SERVICE, KEYRING_API_KEY)
            .load()
            .ok()
            .flatten()
            .map(|c| c.secret)
            .or_else(|| std::env::var(APPLICATION_KEY_ENV).ok())
            .ok_or_else(|| {
                LibraryServiceError::new(
                    LibraryServiceErrorKind::Session,
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

/// Extracts the last page number from a [`PaginationLinks`] `last` URL.
///
/// Parses the `page` query parameter from the URL using a simple string split.
/// Returns `None` if `links.last` is absent or contains no valid `page` value.
fn last_page_from_links(links: &PaginationLinks) -> Option<u32> {
    let last_url = links.last.as_deref()?;
    // Find "page=" in the query string and parse the digits that follow.
    let page_part = last_url.split("page=").nth(1)?;
    let digits: String = page_part
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse::<u32>().ok().filter(|&n| n > 0)
}

fn publisher_lookup(included: &[IncludedItem]) -> HashMap<u64, String> {
    included
        .iter()
        .filter_map(IncludedItem::as_publisher)
        .filter(|publisher| publisher.publisher_id > 0)
        .map(|publisher| (publisher.publisher_id, publisher.name))
        .collect()
}

/// Builds a lookup from JSON:API resource id (e.g. `"/api/vBeta/products/187766"`) to
/// `Product` resource attributes, from one page's `included` array.
///
/// The live API sideloads `Product` (and `Publisher`/`Order`) resources in `included`
/// rather than embedding them on `OrderProductAttributes` directly; each ordered product's
/// `relationships.product.data.id` is the key into this map.
fn product_lookup(included: &[IncludedItem]) -> HashMap<String, OrderProductInfo> {
    included
        .iter()
        .filter_map(|entry| {
            entry
                .as_product()
                .map(|product| (entry.id.clone(), product))
        })
        .collect()
}

/// Derives an uppercase format label (e.g. `"PDF"`, `"EPUB"`) from a file's extension.
///
/// Returns `None` if `filename` has no extension.
fn file_extension_label(filename: &str) -> Option<String> {
    filename
        .rsplit_once('.')
        .map(|(_, ext)| ext.to_ascii_uppercase())
}

fn map_order_product(
    item: &OrderProductItem,
    publishers: &HashMap<u64, String>,
    products: &HashMap<String, OrderProductInfo>,
    order: u32,
) -> LibraryItem {
    let attributes = &item.attributes;

    let numeric_id = attributes
        .order_product_id
        .max(item.id.parse::<u64>().unwrap_or_default())
        .max(attributes.product_id);

    // Prefer the publisher name embedded directly on `attributes.publisher` (present on newer
    // API responses); fall back to the sideloaded `included` publisher lookup, then a placeholder.
    let publisher = attributes
        .publisher
        .as_ref()
        .map(|p| p.name.clone())
        .or_else(|| publishers.get(&attributes.royalty_publisher_id).cloned())
        .unwrap_or_else(|| format!("Publisher {}", attributes.royalty_publisher_id));

    // The live API resolves `Product` metadata (cover images) via `relationships.product`
    // against the response's sideloaded `included` array rather than embedding it directly
    // on `attributes` — fall back to the embedded field in case a future/legacy response
    // shape does embed it inline.
    let product_info = item
        .relationships
        .as_ref()
        .and_then(|r| r.product.as_ref())
        .and_then(|r| r.data.as_ref())
        .and_then(|d| products.get(&d.id))
        .or(attributes.product.as_ref());

    // Prefer the smallest thumbnail available for catalog rendering, falling back to
    // progressively larger images if a thumbnail wasn't generated for this product.
    let cover_url = product_info.and_then(|p| {
        p.thumbnail
            .as_deref()
            .or(p.thumbnail_100.as_deref())
            .or(p.image.as_deref())
            .map(|path| format!("{DTRPG_IMAGES_BASE_URL}{path}"))
    });

    let kind = attributes
        .filters
        .as_ref()
        .and_then(|filters| filters.iter().find(|f| f.parent_filter_id == 0))
        .map(|f| {
            if f.parent_name.is_empty() {
                f.name.clone()
            } else {
                f.parent_name.clone()
            }
        })
        .unwrap_or_else(|| "Library item".to_string());

    // File `title` is the document's display name (e.g. "Player's Handbook"), not a
    // format type — derive the format from the file extension instead.
    let mut format_parts: Vec<String> = attributes
        .files
        .iter()
        .filter_map(|f| file_extension_label(&f.filename))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    format_parts.sort();
    let format = if format_parts.is_empty() {
        "PDF".to_string()
    } else {
        format_parts.join(" + ")
    };

    let size_mb = attributes.files.iter().map(|f| f.size as f64).sum::<f64>() / BYTES_PER_MB;

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
        order_product_id: attributes.order_product_id,
        product_id: attributes.product_id,
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
        cover_url: cover_url.map(Into::into),
        date_added: None,
        thumbnail_last_attempted: None,
    }
}

fn map_client_error(error: ClientError) -> LibraryServiceError {
    match error {
        ClientError::Sdk(error) => map_sdk_error(error),
        ClientError::DecodeFailed {
            url, status, cause, ..
        } => {
            let kind = match status {
                401 => LibraryServiceErrorKind::NeedsReauth,
                403 => LibraryServiceErrorKind::Session,
                _ => LibraryServiceErrorKind::Network,
            };
            LibraryServiceError::new(
                kind,
                format!("Response from {url} (HTTP {status}) could not be decoded: {cause}"),
            )
        }
        ClientError::Http(error) => {
            let status = error.status().map(|s| s.as_u16());
            let kind = match status {
                Some(401) => LibraryServiceErrorKind::NeedsReauth,
                Some(403) => LibraryServiceErrorKind::Session,
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
        FileChecksum, OrderProductAttributes, OrderProductFile, OrderProductRelationships,
        PaginationLinks, PaginationMeta, RelationshipData, RelationshipRef,
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
                    included: Some(vec![IncludedItem {
                        id: "/api/vBeta/publishers/7".to_string(),
                        resource_type: "Publisher".to_string(),
                        attributes: serde_json::json!({
                            "name": "Lantern Press",
                            "publisherId": 7,
                            "slug": "lantern-press",
                        }),
                    }]),
                }),
                detail_result: Ok(OrderProductItemResponse { data: item }),
            }
        }

        fn seeded_with(item: OrderProductItem) -> Self {
            Self {
                list_result: Ok(OrderProductListResponse {
                    links: pagination_links(None),
                    meta: PaginationMeta {
                        items_per_page: 100,
                        current_page: 1,
                    },
                    data: vec![item.clone()],
                    included: Some(vec![IncludedItem {
                        id: "/api/vBeta/publishers/7".to_string(),
                        resource_type: "Publisher".to_string(),
                        attributes: serde_json::json!({
                            "name": "Lantern Press",
                            "publisherId": 7,
                            "slug": "lantern-press",
                        }),
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
                    meta: PaginationMeta {
                        items_per_page: 1,
                        current_page: 1,
                    },
                    data: vec![order_product_item(42, "Item Page One")],
                    included: None,
                })
            } else {
                Ok(OrderProductListResponse {
                    links: pagination_links(None),
                    meta: PaginationMeta {
                        items_per_page: 1,
                        current_page: 2,
                    },
                    data: vec![order_product_item(99, "Item Page Two")],
                    included: None,
                })
            }
        }

        fn get_order_product(
            &self,
            _id: u64,
        ) -> Result<OrderProductItemResponse, LibraryServiceError> {
            Err(LibraryServiceError::new(
                LibraryServiceErrorKind::NotFound,
                "not used",
            ))
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
    fn map_order_product_derives_format_from_file_extension_not_title() {
        let mut item = order_product_item(515_276, "The Wellspring");
        // File `title` is the document's display name, distinct from its extension —
        // the mapped format must come from the extension, not this field.
        item.attributes.files = vec![OrderProductFile {
            index: 0,
            order_product_download_id: 1234,
            title: "The Wellspring".to_string(),
            filename: "the-wellspring.epub".to_string(),
            size: 1_048_576,
            size_mb: "1.0".to_string(),
            checksums: vec![],
        }];

        let service = RustSdkLibraryService::new(Box::new(FakeSdkGateway::seeded_with(item)));
        let items = service.list_items().expect("list items");

        assert_eq!(items[0].format.as_ref(), "EPUB");
    }

    #[test]
    fn map_order_product_joins_multiple_distinct_extensions() {
        let mut item = order_product_item(515_276, "The Wellspring");
        item.attributes.files = vec![
            OrderProductFile {
                index: 0,
                order_product_download_id: 1234,
                title: "The Wellspring".to_string(),
                filename: "the-wellspring.pdf".to_string(),
                size: 1_048_576,
                size_mb: "1.0".to_string(),
                checksums: vec![],
            },
            OrderProductFile {
                index: 1,
                order_product_download_id: 1235,
                title: "The Wellspring".to_string(),
                filename: "the-wellspring.epub".to_string(),
                size: 1_048_576,
                size_mb: "1.0".to_string(),
                checksums: vec![],
            },
        ];

        let service = RustSdkLibraryService::new(Box::new(FakeSdkGateway::seeded_with(item)));
        let items = service.list_items().expect("list items");

        assert_eq!(items[0].format.as_ref(), "EPUB + PDF");
    }

    #[test]
    fn map_order_product_builds_cover_url_from_sideloaded_product_relationship() {
        // Matches the live API's actual shape: `product` metadata is *not* embedded on
        // `attributes` — it's referenced via `relationships.product.data.id` and resolved
        // against the response's `included` array.
        let mut item = order_product_item(515_276, "The Wellspring");
        item.attributes.royalty_publisher_id = 4952;
        item.relationships = Some(OrderProductRelationships {
            publisher: None,
            product: Some(RelationshipRef {
                data: Some(RelationshipData {
                    resource_type: "Product".to_string(),
                    id: "/api/vBeta/products/515276".to_string(),
                }),
            }),
            order: None,
        });

        let mut products = HashMap::new();
        products.insert(
            "/api/vBeta/products/515276".to_string(),
            OrderProductInfo {
                image: Some("4952/515276.jpg".to_string()),
                web_image: Some("4952/515276.webp".to_string()),
                thumbnail: Some("4952/515276-thumb140.jpg".to_string()),
                thumbnail_100: Some("4952/515276-thumb100.jpg".to_string()),
                bundle_id: 0,
                date_created: Some("2025-03-13T16:07:01-05:00".to_string()),
                product_id: 515_276,
                description: None,
                filesize: Some(24.13),
            },
        );

        let mut publishers = HashMap::new();
        publishers.insert(4952, "Monte Cook Games".to_string());

        let mapped = map_order_product(&item, &publishers, &products, 0);

        assert_eq!(mapped.publisher.as_ref(), "Monte Cook Games");
        assert_eq!(
            mapped.cover_url.as_deref(),
            Some("https://api.drivethrurpg.com/images/4952/515276-thumb140.jpg")
        );
    }

    #[test]
    fn map_order_product_builds_cover_url_from_embedded_thumbnail_fallback() {
        // Defensive fallback path: if `product` were ever embedded directly on `attributes`
        // (e.g. a future/legacy response shape), it should still resolve without a
        // `relationships`/`included` sideload.
        let mut item = order_product_item(515_276, "The Wellspring");
        item.attributes.royalty_publisher_id = 4952;
        item.attributes.publisher = Some(dtrpg_sdk::OrderProductPublisher {
            name: "Monte Cook Games".to_string(),
            publisher_id: 4952,
            slug: "monte-cook-games".to_string(),
        });
        item.attributes.product = Some(OrderProductInfo {
            image: Some("4952/515276.jpg".to_string()),
            web_image: Some("4952/515276.webp".to_string()),
            thumbnail: Some("4952/515276-thumb140.jpg".to_string()),
            thumbnail_100: Some("4952/515276-thumb100.jpg".to_string()),
            bundle_id: 0,
            date_created: Some("2025-03-13T16:07:01-05:00".to_string()),
            product_id: 515_276,
            description: None,
            filesize: Some(24.13),
        });

        let mapped = map_order_product(&item, &HashMap::new(), &HashMap::new(), 0);

        assert_eq!(mapped.publisher.as_ref(), "Monte Cook Games");
        assert_eq!(
            mapped.cover_url.as_deref(),
            Some("https://api.drivethrurpg.com/images/4952/515276-thumb140.jpg")
        );
    }

    #[test]
    fn map_order_product_falls_back_to_publisher_lookup_when_not_embedded() {
        let item = order_product_item(7, "No Embedded Publisher");
        let mut publishers = HashMap::new();
        publishers.insert(7, "Lantern Press".to_string());

        let mapped = map_order_product(&item, &publishers, &HashMap::new(), 0);

        assert_eq!(mapped.publisher.as_ref(), "Lantern Press");
        assert!(mapped.cover_url.is_none());
    }

    #[test]
    fn count_items_uses_last_page_number_when_link_present() {
        let mut gateway = FakeSdkGateway::seeded();
        gateway.list_result = Ok(OrderProductListResponse {
            links: PaginationLinks {
                self_: "self".to_string(),
                first: None,
                last: Some(
                    "https://api.example.com/order_products?pageSize=1&page=137".to_string(),
                ),
                prev: None,
                next: None,
            },
            meta: PaginationMeta {
                items_per_page: 1,
                current_page: 1,
            },
            data: vec![order_product_item(1, "First Item")],
            included: None,
        });
        let service = RustSdkLibraryService::new(Box::new(gateway));

        let count = service.count_items().expect("count supported");

        assert_eq!(count.expect("count succeeds"), 137);
    }

    #[test]
    fn count_items_falls_back_to_page_length_without_last_link() {
        let service = RustSdkLibraryService::new(Box::new(FakeSdkGateway::seeded()));

        let count = service.count_items().expect("count supported");

        assert_eq!(count.expect("count succeeds"), 1);
    }

    #[test]
    fn count_items_propagates_gateway_error() {
        let service = RustSdkLibraryService::new(Box::new(FakeSdkGateway::session_error()));

        let count = service.count_items().expect("count supported");

        assert_eq!(
            count.expect_err("session error propagated").kind,
            LibraryServiceErrorKind::Session
        );
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
                publisher: None,
                product: None,
                order: None,
            },
            relationships: None,
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
    fn last_page_from_links_parses_page_number() {
        let links = PaginationLinks {
            self_: "self".to_string(),
            first: None,
            last: Some("https://api.example.com/orders?page=42&pageSize=100".to_string()),
            prev: None,
            next: None,
        };
        assert_eq!(last_page_from_links(&links), Some(42));
    }

    #[test]
    fn last_page_from_links_returns_none_when_absent() {
        let links = pagination_links(None);
        assert_eq!(last_page_from_links(&links), None);
    }

    #[test]
    fn last_page_from_links_returns_none_when_no_page_param() {
        let links = PaginationLinks {
            self_: "self".to_string(),
            first: None,
            last: Some("https://api.example.com/orders?pageSize=100".to_string()),
            prev: None,
            next: None,
        };
        assert_eq!(last_page_from_links(&links), None);
    }

    #[test]
    fn sdk_service_fetches_all_pages_via_pagination() {
        let gateway = TwoPageGateway {
            call_count: AtomicU32::new(0),
        };
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
