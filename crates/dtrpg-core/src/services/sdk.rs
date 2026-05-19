// //! Rust SDK-backed implementation of [`LibraryService`].

// use std::collections::HashMap;

// use dtrpg_sdk::{
//     AuthTokenResponse, ClientError, Config, DriveThruRpgSdk, LibraryClient as SdkLibraryClient,
//     LibraryItemsParams, OrderProductItem, OrderProductItemResponse, OrderProductListResponse,
//     PublisherItem, SdkError,
// };
// use tokio::runtime::{Builder, Runtime};

// use super::{LibraryItem, LibraryService, LibraryServiceError, LibraryServiceErrorKind};

// const APPLICATION_KEY_ENV: &str = "DTRPG_APPLICATION_KEY";
// const ACCESS_TOKEN_ENV: &str = "DTRPG_ACCESS_TOKEN";
// const REFRESH_TOKEN_ENV: &str = "DTRPG_REFRESH_TOKEN";
// const REFRESH_TOKEN_TTL_ENV: &str = "DTRPG_REFRESH_TOKEN_TTL";
// const API_BASE_URL_ENV: &str = "DTRPG_API_BASE_URL";

// /// SDK operation boundary used by the Rust library service adapter.
// pub trait SdkLibraryGateway: Send + Sync {
//     /// Lists ordered products through the Rust SDK.
//     fn list_order_products(
//         &self,
//         params: LibraryItemsParams,
//     ) -> Result<OrderProductListResponse, LibraryServiceError>;

//     /// Loads an ordered product detail through the Rust SDK.
//     fn get_order_product(&self, id: u64) -> Result<OrderProductItemResponse, LibraryServiceError>;
// }

// /// Library service adapter backed by the Rust SDK.
// pub struct RustSdkLibraryService {
//     gateway: Box<dyn SdkLibraryGateway>,
// }

// impl RustSdkLibraryService {
//     /// Creates a service from an SDK gateway implementation.
//     pub fn new(gateway: Box<dyn SdkLibraryGateway>) -> Self {
//         Self { gateway }
//     }

//     /// Creates the default service from environment-provided SDK configuration and session.
//     pub fn from_environment() -> Self {
//         match HttpSdkLibraryGateway::from_environment() {
//             Ok(gateway) => Self::new(Box::new(gateway)),
//             Err(error) => Self::new(Box::new(UnavailableSdkGateway::new(error))),
//         }
//     }
// }

// impl LibraryService for RustSdkLibraryService {
//     fn list_items(&self) -> Result<Vec<LibraryItem>, LibraryServiceError> {
//         let params = LibraryItemsParams {
//             page: Some(1),
//             page_size: Some(100),
//             get_checksum: Some(false),
//             get_filters: Some(true),
//             library: Some(true),
//             archived: Some(false),
//             updated_date_after: None,
//         };

//         let response = self.gateway.list_order_products(params)?;
//         let publishers = publisher_lookup(response.included.as_deref().unwrap_or(&[]));

//         Ok(response
//             .data
//             .iter()
//             .enumerate()
//             .map(|(index, item)| map_order_product(item, &publishers, index as u32))
//             .collect())
//     }

//     fn get_item(&self, id: u64) -> Result<LibraryItem, LibraryServiceError> {
//         let response = self.gateway.get_order_product(id)?;
//         Ok(map_order_product(&response.data, &HashMap::new(), 0))
//     }
// }

// struct HttpSdkLibraryGateway {
//     client: SdkLibraryClient,
//     runtime: Runtime,
// }

// impl HttpSdkLibraryGateway {
//     fn from_environment() -> Result<Self, LibraryServiceError> {
//         let application_key = std::env::var(APPLICATION_KEY_ENV).map_err(|_| {
//             LibraryServiceError::new(
//                 LibraryServiceErrorKind::Network,
//                 format!(
//                     "{APPLICATION_KEY_ENV} is required before loading SDK-backed library data."
//                 ),
//             )
//         })?;
//         let access_token = std::env::var(ACCESS_TOKEN_ENV).map_err(|_| {
//             LibraryServiceError::new(
//                 LibraryServiceErrorKind::Session,
//                 format!("{ACCESS_TOKEN_ENV} is required before loading SDK-backed library data."),
//             )
//         })?;
//         let refresh_token = std::env::var(REFRESH_TOKEN_ENV).unwrap_or_default();
//         let refresh_token_ttl = std::env::var(REFRESH_TOKEN_TTL_ENV)
//             .ok()
//             .and_then(|value| value.parse::<u64>().ok())
//             .unwrap_or(u64::MAX);

//         let config = match std::env::var(API_BASE_URL_ENV) {
//             Ok(base_url) => Config::with_base_url(application_key, base_url),
//             Err(_) => Config::new(application_key),
//         };

//         let mut sdk = DriveThruRpgSdk::with_config(config);
//         sdk.apply_auth_response(AuthTokenResponse::new(
//             access_token,
//             refresh_token,
//             refresh_token_ttl,
//         ))
//         .map_err(map_sdk_error)?;

//         let client = sdk.library_client().map_err(map_sdk_error)?;
//         let runtime = Builder::new_multi_thread()
//             .enable_all()
//             .build()
//             .map_err(|error| {
//                 LibraryServiceError::new(
//                     LibraryServiceErrorKind::Network,
//                     format!("Unable to start Rust SDK runtime: {error}"),
//                 )
//             })?;

//         Ok(Self { client, runtime })
//     }
// }

// impl SdkLibraryGateway for HttpSdkLibraryGateway {
//     fn list_order_products(
//         &self,
//         params: LibraryItemsParams,
//     ) -> Result<OrderProductListResponse, LibraryServiceError> {
//         self.runtime
//             .block_on(self.client.list_order_products(params))
//             .map_err(map_client_error)
//     }

//     fn get_order_product(&self, id: u64) -> Result<OrderProductItemResponse, LibraryServiceError> {
//         self.runtime
//             .block_on(self.client.get_order_product(id))
//             .map_err(map_client_error)
//     }
// }

// struct UnavailableSdkGateway {
//     error: LibraryServiceError,
// }

// impl UnavailableSdkGateway {
//     fn new(error: LibraryServiceError) -> Self {
//         Self { error }
//     }
// }

// impl SdkLibraryGateway for UnavailableSdkGateway {
//     fn list_order_products(
//         &self,
//         _params: LibraryItemsParams,
//     ) -> Result<OrderProductListResponse, LibraryServiceError> {
//         Err(self.error.clone())
//     }

//     fn get_order_product(&self, _id: u64) -> Result<OrderProductItemResponse, LibraryServiceError> {
//         Err(self.error.clone())
//     }
// }

// fn publisher_lookup(included: &[PublisherItem]) -> HashMap<u64, String> {
//     included
//         .iter()
//         .map(|publisher| {
//             (
//                 publisher.attributes.publisher_id,
//                 publisher.attributes.name.clone(),
//             )
//         })
//         .collect()
// }

// fn map_order_product(
//     item: &OrderProductItem,
//     publishers: &HashMap<u64, String>,
//     order: u32,
// ) -> LibraryItem {
//     let attributes = &item.attributes;
//     let id = attributes
//         .order_product_id
//         .max(item.id.parse::<u64>().unwrap_or_default())
//         .max(attributes.product_id);
//     let publisher = publishers
//         .get(&attributes.royalty_publisher_id)
//         .cloned()
//         .unwrap_or_else(|| format!("Publisher {}", attributes.royalty_publisher_id));
//     let product_type = attributes
//         .filters
//         .as_ref()
//         .and_then(|filters| filters.first())
//         .map(|filter| filter.parent_name.clone())
//         .filter(|name| !name.is_empty())
//         .unwrap_or_else(|| "Library item".to_string());

//     LibraryItem {
//         id,
//         title: attributes.name.clone(),
//         publisher,
//         product_type,
//         added_order: order,
//         updated_order: order,
//         summary: summary_for_order_product(item),
//     }
// }

// fn summary_for_order_product(item: &OrderProductItem) -> String {
//     let attributes = &item.attributes;
//     let mut parts = Vec::new();

//     if let Some(date) = &attributes.date_purchased {
//         parts.push(format!("Purchased {date}"));
//     }
//     if let Some(date) = &attributes.file_last_modified {
//         parts.push(format!("Updated {date}"));
//     }
//     parts.push(format!("{} downloadable file(s)", attributes.files.len()));
//     parts.push(format!("Final price {:.2}", attributes.final_price));

//     parts.join(". ")
// }

// fn map_client_error(error: ClientError) -> LibraryServiceError {
//     match error {
//         ClientError::Sdk(error) => map_sdk_error(error),
//         ClientError::Http(error) => {
//             let kind = match error.status().map(|status| status.as_u16()) {
//                 Some(401 | 403) => LibraryServiceErrorKind::Session,
//                 _ => LibraryServiceErrorKind::Network,
//             };

//             LibraryServiceError::new(kind, format!("Rust SDK library request failed: {error}"))
//         }
//     }
// }

// fn map_sdk_error(error: SdkError) -> LibraryServiceError {
//     let kind = match error {
//         SdkError::Unauthenticated | SdkError::AuthSession(_) => LibraryServiceErrorKind::Session,
//         SdkError::Unconfigured => LibraryServiceErrorKind::Network,
//     };

//     LibraryServiceError::new(kind, format!("Rust SDK is not ready: {error}"))
// }

// #[cfg(test)]
// mod tests {
//     use dtrpg_sdk::{
//         FileChecksum, OrderProductAttributes, OrderProductFile, PaginationLinks, PaginationMeta,
//         PublisherAttributes,
//     };

//     use super::*;

//     struct FakeSdkGateway {
//         list_result: Result<OrderProductListResponse, LibraryServiceError>,
//         detail_result: Result<OrderProductItemResponse, LibraryServiceError>,
//     }

//     impl FakeSdkGateway {
//         fn seeded() -> Self {
//             let item = order_product_item(42, "A Better Dungeon");
//             Self {
//                 list_result: Ok(OrderProductListResponse {
//                     links: pagination_links(),
//                     meta: PaginationMeta {
//                         items_per_page: 100,
//                         current_page: 1,
//                     },
//                     data: vec![item.clone()],
//                     included: Some(vec![PublisherItem {
//                         id: "7".to_string(),
//                         resource_type: "publisher".to_string(),
//                         attributes: PublisherAttributes {
//                             name: "Lantern Press".to_string(),
//                             publisher_id: 7,
//                             slug: "lantern-press".to_string(),
//                         },
//                     }]),
//                 }),
//                 detail_result: Ok(OrderProductItemResponse { data: item }),
//             }
//         }

//         fn session_error() -> Self {
//             let error = LibraryServiceError::new(
//                 LibraryServiceErrorKind::Session,
//                 "SDK session expired in test.",
//             );
//             Self {
//                 list_result: Err(error.clone()),
//                 detail_result: Err(error),
//             }
//         }
//     }

//     impl SdkLibraryGateway for FakeSdkGateway {
//         fn list_order_products(
//             &self,
//             _params: LibraryItemsParams,
//         ) -> Result<OrderProductListResponse, LibraryServiceError> {
//             self.list_result.clone()
//         }

//         fn get_order_product(
//             &self,
//             _id: u64,
//         ) -> Result<OrderProductItemResponse, LibraryServiceError> {
//             self.detail_result.clone()
//         }
//     }

//     #[test]
//     fn sdk_service_maps_order_products_to_library_items() {
//         let service = RustSdkLibraryService::new(Box::new(FakeSdkGateway::seeded()));

//         let items = service.list_items().expect("list items");

//         assert_eq!(items.len(), 1);
//         assert_eq!(items[0].id, 42);
//         assert_eq!(items[0].title, "A Better Dungeon");
//         assert_eq!(items[0].publisher, "Lantern Press");
//         assert_eq!(items[0].product_type, "Adventure");
//     }

//     #[test]
//     fn sdk_service_preserves_session_error_classification() {
//         let service = RustSdkLibraryService::new(Box::new(FakeSdkGateway::session_error()));

//         let error = service.list_items().expect_err("session error");

//         assert_eq!(error.kind, LibraryServiceErrorKind::Session);
//     }

//     fn order_product_item(id: u64, name: &str) -> OrderProductItem {
//         OrderProductItem {
//             id: id.to_string(),
//             resource_type: "order_product".to_string(),
//             attributes: OrderProductAttributes {
//                 order_id: 900,
//                 product_id: id,
//                 royalty_publisher_id: 7,
//                 isbn: None,
//                 name: name.to_string(),
//                 date_purchased: Some("2026-01-01".to_string()),
//                 filesize: Some(1024),
//                 final_price: 12.5,
//                 quantity: 1,
//                 bundle_id: 0,
//                 archived: 0,
//                 add_on_info: None,
//                 order_product_id: id,
//                 customer_id: 123,
//                 file_last_modified: Some("2026-01-02".to_string()),
//                 file_last_downloaded: None,
//                 files: vec![OrderProductFile {
//                     index: 0,
//                     order_product_download_id: 1234,
//                     title: "PDF".to_string(),
//                     filename: "better-dungeon.pdf".to_string(),
//                     size: 1024,
//                     size_mb: "0.001".to_string(),
//                     checksums: vec![FileChecksum {
//                         checksum: "abc123".to_string(),
//                         checksum_date: "2026-01-02".to_string(),
//                     }],
//                 }],
//                 filters: Some(vec![dtrpg_sdk::OrderProductFilter {
//                     filter_id: 1,
//                     parent_filter_id: 0,
//                     name: "Dungeon".to_string(),
//                     parent_name: "Adventure".to_string(),
//                 }]),
//                 history: None,
//                 attributes: None,
//             },
//         }
//     }

//     fn pagination_links() -> PaginationLinks {
//         PaginationLinks {
//             self_: "self".to_string(),
//             first: None,
//             last: None,
//             prev: None,
//             next: None,
//         }
//     }
// }
