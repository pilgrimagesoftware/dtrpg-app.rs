//! Prints the filter categories DriveThruRPG attaches to your library's
//! products, to find which of them carry the game system and the content type.
//!
//! It signs in with the application key Libri already stores in the system
//! keyring (or `DTRPG_APPLICATION_KEY`), lists your library with filters
//! included, and prints JSON on standard output:
//!
//! - `filter_tree`: every filter category grouped under its parent, with how
//!   many products have it.
//! - `products`: the filters (and purchase attributes) of the first `--sample`
//!   products.
//!
//! Product titles are left out unless you pass `--titles`, so the output can be
//! shared. Nothing is written to disk and nothing is changed in your account.
//!
//! ```text
//! cargo run -p dtrpg-core --example dump_filters -- --sample 25
//! ```
//!
//! Options: `--sample N` (default 25, `0` for every product) and `--titles`.

use std::collections::BTreeMap;
use std::error::Error;
use std::future::Future;
use std::time::Duration;

use dtrpg_core::constants::{API_BASE_URL_ENV, APPLICATION_KEY_ENV};
use dtrpg_core::credentials::{CredentialStore, KeyringCredentialStore};
use dtrpg_core::data::constants::{KEYRING_API_KEY, KEYRING_SERVICE};
use dtrpg_sdk::auth::key_exchange;
use dtrpg_sdk::{Config, DriveThruRpgSdk, LibraryItemsParams, OrderProductItem};
use serde_json::json;

/// How many products to ask for per page.
const PAGE_SIZE: u32 = 100;

/// How long one step may take before the example gives up and says which.
const STEP_TIMEOUT: Duration = Duration::from_secs(30);

/// Waits for `step` on the runtime, giving up after [`STEP_TIMEOUT`] so that a
/// website that does not answer (for example, while it is down for
/// maintenance) ends in a message instead of a silent hang.
fn run_step<T, E>(runtime: &tokio::runtime::Handle, name: &str,
                  step: impl Future<Output = Result<T, E>>)
                  -> Result<T, Box<dyn Error>>
    where E: Error + 'static {
    eprintln!("{name}...");
    match runtime.block_on(tokio::time::timeout(STEP_TIMEOUT, step)) {
        Ok(result) => Ok(result?),
        Err(_) => Err(format!("{name}: no answer after {} seconds; the DriveThruRPG site may \
                               be down",
                              STEP_TIMEOUT.as_secs()).into()),
    }
}

/// What to print, from the command line.
struct Options {
    /// How many products' filters to list in full; `0` means all of them.
    sample: usize,
    /// Whether to include product titles.
    titles: bool,
}

/// Reads `--sample N` and `--titles` from the arguments.
fn parse_options(args: impl Iterator<Item = String>) -> Result<Options, String> {
    let mut options = Options { sample: 25,
                                titles: false, };
    let mut args = args;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--titles" => options.titles = true,
            "--sample" => {
                let value = args.next().ok_or("--sample needs a number")?;
                options.sample = value.parse()
                                      .map_err(|_| format!("--sample: not a number: {value}"))?;
            }
            other => return Err(format!("unknown option: {other}")),
        }
    }
    Ok(options)
}

/// The application key from the keyring, or from the environment.
fn application_key() -> Result<String, Box<dyn Error>> {
    eprintln!("Reading the application key from the system keyring (an unlock prompt may \
               appear)...");
    let stored = KeyringCredentialStore::new(KEYRING_SERVICE, KEYRING_API_KEY).load()
                                                                              .ok()
                                                                              .flatten()
                                                                              .map(|credential| {
                                                                                  credential.secret
                                                                              });
    stored.or_else(|| std::env::var(APPLICATION_KEY_ENV).ok())
          .ok_or_else(|| {
              format!("no application key found: sign in to Libri first, or set \
                       {APPLICATION_KEY_ENV}").into()
          })
}

/// Lists every product in the library, with its filters.
fn list_products(application_key: &str) -> Result<Vec<OrderProductItem>, Box<dyn Error>> {
    let config = match std::env::var(API_BASE_URL_ENV) {
        Ok(base_url) => Config::with_base_url(application_key.to_owned(), base_url),
        Err(_) => Config::new(application_key.to_owned()),
    };
    let runtime = dtrpg_core::runtime::handle()?;
    let tokens = run_step(&runtime,
                          "Signing in",
                          key_exchange::authenticate(application_key, &config))?;
    let mut sdk = DriveThruRpgSdk::with_config(config);
    sdk.apply_auth_response(tokens)?;
    let client = sdk.library_client()?;

    let mut products = Vec::new();
    for page in 1.. {
        let params = LibraryItemsParams { page:               Some(page),
                                          page_size:          Some(PAGE_SIZE),
                                          get_checksum:       Some(false),
                                          get_filters:        Some(true),
                                          library:            Some(true),
                                          archived:           Some(false),
                                          updated_date_after: None, };
        let response = run_step(&runtime,
                                &format!("Listing library page {page}"),
                                client.list_order_products(params))?;
        let last_page = response.links.next.is_none();
        products.extend(response.data);
        if last_page {
            break;
        }
    }
    Ok(products)
}

/// Groups every filter under its parent category, counting products.
fn filter_tree(products: &[OrderProductItem]) -> serde_json::Value {
    // parent name -> (filter id, name) -> number of products
    let mut tree: BTreeMap<String, BTreeMap<(u64, String), usize>> = BTreeMap::new();
    for filters in products.iter()
                           .filter_map(|product| product.attributes.filters.as_ref())
    {
        for filter in filters {
            *tree.entry(filter.parent_name.clone())
                 .or_default()
                 .entry((filter.filter_id, filter.name.clone()))
                 .or_default() += 1;
        }
    }
    tree.into_iter()
        .map(|(parent, children)| {
            let children: Vec<_> = children.into_iter()
                                           .map(|((filter_id, name), products)| {
                                               json!({ "filter_id": filter_id,
                                                       "name": name,
                                                       "products": products })
                                           })
                                           .collect();
            json!({ "parent": parent, "filters": children })
        })
        .collect()
}

/// One product's filters and purchase attributes.
fn describe(product: &OrderProductItem, titles: bool) -> serde_json::Value {
    let attributes = &product.attributes;
    let mut described = json!({ "product_id": attributes.product_id,
                                "order_product_id": attributes.order_product_id,
                                "filters": attributes.filters,
                                "attributes": attributes.attributes });
    if titles {
        described["title"] = json!(attributes.name);
    }
    described
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = parse_options(std::env::args().skip(1))?;
    let products = list_products(&application_key()?)?;
    let sample = if options.sample == 0 {
        products.len()
    }
    else {
        options.sample
    };
    let output = json!({
        "products_seen": products.len(),
        "filter_tree": filter_tree(&products),
        "products": products.iter()
                            .take(sample)
                            .map(|product| describe(product, options.titles))
                            .collect::<Vec<_>>(),
    });
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
