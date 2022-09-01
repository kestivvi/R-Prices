use database::models::offer::Offer;
use database::models::price::{Availability, CreatePriceInput, Price};
use database::models::product::Product;
use diesel::PgConnection;
use email::email_many;
use log::{debug, error};
use price_scraper::{GetPriceError, PriceScraper};

///////////////////////////////////////////////////////////////////////////////
// Modules Declaration

pub mod config;
pub mod email;
pub mod price_scraper;

///////////////////////////////////////////////////////////////////////////////
// Structures

// #[derive(Default)]
// struct Stats {
//     pub all: u64,
//     pub success: u64,
//     pub price_not_found: u64,
//     pub redirected: u64,
//     pub other_error: u64,
//     pub page_not_supported: u64,
// }

// impl Stats {
//     fn done(&self) -> u64 {
//         &self.success
//             + &self.price_not_found
//             + &self.redirected
//             + &self.other_error
//             + &self.page_not_supported
//     }
// }

struct OfferToUpdate {
    offer: Offer,
    last_price: Price,
    products: Vec<Product>,
}

///////////////////////////////////////////////////////////////////////////////
// Errors

#[derive(thiserror::Error, Debug)]
#[error("Error updating the offer")]
enum UpdateOfferError {
    PageNotSupported,
    DatabaseError,
    // ErrorDownloadingPage,
    // ErrorDownloadingPageDueToTimeout,
    // PageNotFound,
}

///////////////////////////////////////////////////////////////////////////////
// Public Functions

pub async fn update_all_offers(scraper: &PriceScraper, conn: &PgConnection) {
    // Initialize Stats struct
    // let stats = Arc::new(Mutex::new(Stats::default()));

    //// Prepare data for tasks
    // Get all offers from database
    let offers = database::models::offer::queries::all_offers(&conn).unwrap();

    // Get last prices of offers
    let last_prices: Vec<Price> = offers
        .iter()
        .map(|offer| {
            let last_price =
                database::models::price::queries::get_last_prices_of_offer(&conn, offer.id, 1);
            last_price.into_iter().next()
        })
        .flatten()
        .flatten()
        .collect();

    let products: Vec<Vec<Product>> = offers
        .iter()
        .map(|offer| {
            let products = database::models::offer::queries::get_products_of_offer(&conn, offer.id);
            products.ok()
        })
        .flatten()
        .collect();

    use itertools::izip;
    let offers_to_update =
        izip!(offers, last_prices, products).map(|(offer, last_price, products)| OfferToUpdate {
            offer,
            last_price,
            products,
        });

    // Construct on_update closure
    // stats.lock().unwrap().all = offers_to_update.len() as u64;
    // let stats_clone = Arc::clone(&stats);
    // let on_update = || {
    //     let all;
    //     let done;
    //     {
    //         let stats_lock = stats_clone.lock().unwrap();
    //         all = stats_lock.all;
    //         done = stats_lock.done();
    //         info!("Updated: {done}/{all}");
    //     }
    // };

    // Get handles to async tasks
    let handles = offers_to_update.map(|offer| update_price_of_offer(scraper, conn, offer));

    // Run asynchronously
    futures::future::join_all(handles).await;

    // Print info stats at the end
    //     let stats_lock = stats.lock().unwrap();
    //     let all = stats_lock.all;
    //     let done = stats_lock.done();

    //     info!(
    //         "
    // Updated {}/{}:
    //     - {} successfully updated
    //     - {} got redirected away (page not found)
    //     - {} price not found on given page (product unavailable probably)
    //     - {} other error while downloading occured
    //     - {} not supported pages",
    //         done,
    //         all,
    //         stats_lock.success,
    //         stats_lock.redirected,
    //         stats_lock.price_not_found,
    //         stats_lock.other_error,
    //         stats_lock.page_not_supported
    //     )
}

async fn update_price_of_offer(scraper: &PriceScraper, conn: &PgConnection, data: OfferToUpdate) {
    // Try get price
    // TODO: Make fn get_price with parameters, u32 tries, u32 fairness_tries
    let price_result = scraper.get_price_multiple_tries(&data.offer.url, 3).await;

    // Handle result
    let new_price = match price_result {
        Ok(v) => {
            // debug!("Got proper price for: {}", offer.url);
            // stats.lock().unwrap().success += 1;
            CreatePriceInput {
                offer_id: data.offer.id,
                value: Some(v),
                availability: Availability::Available,
            }
        }
        Err(error) => {
            // debug!("Couldn't get price for: {}\n{:?}", offer.url, error);
            match error.current_context() {
                GetPriceError::PriceNotFound => {
                    // stats.lock().unwrap().price_not_found += 1;
                    CreatePriceInput {
                        offer_id: data.offer.id,
                        value: None,
                        availability: Availability::PriceNotFound,
                    }
                }
                GetPriceError::Redirected => {
                    // stats.lock().unwrap().redirected += 1;
                    CreatePriceInput {
                        offer_id: data.offer.id,
                        value: None,
                        availability: Availability::SiteNotFound,
                    }
                }
                GetPriceError::ErrorDownloadingPage | GetPriceError::PageDownloadTimeout => {
                    // stats.lock().unwrap().other_error += 1;
                    CreatePriceInput {
                        offer_id: data.offer.id,
                        value: None,
                        availability: Availability::Unavailable,
                    }
                }
                GetPriceError::PageNotSupported => {
                    // stats.lock().unwrap().page_not_supported += 1;
                    error!(
                        "\n{:?}",
                        error
                            .change_context(UpdateOfferError::PageNotSupported)
                            .attach_printable(
                                "Tried to update offer but it seems, page is not supported"
                            )
                            .attach_printable(format!("Offer: {:?}", data.offer))
                    );
                    return;
                }
            }
        }
    };

    // Send request to database
    let db_response = database::models::price::mutations::create_price(conn, &new_price);

    // Handle response from database
    let new_price = match db_response {
        Ok(v) => v,
        Err(err) => {
            error!(
                "\n{}",
                error_stack::report!(UpdateOfferError::DatabaseError)
                    .attach_printable("Error trying to insert new price to database")
                    .attach_printable(format!("Offer: {:?}", data.offer))
                    .attach_printable(format!("New price to insert: {:?}", new_price))
                    .attach_printable(format!("Cause: {:?}", err))
            );
            return;
        }
    };

    debug!(
        "Updated: {:?}, {:?} | {}",
        new_price.availability, new_price.value, data.offer.url
    );

    if should_send_notification(&data.last_price, &new_price) {
        send_notification(conn, &data, &new_price);
    }
}

fn should_send_notification(old_price: &Price, new_price: &Price) -> bool {
    if new_price.availability == Availability::Available {
        if old_price.availability == Availability::Available {
            return new_price.value.unwrap() < old_price.value.unwrap();
        } else {
            return true;
        }
    }
    return false;
}

fn send_notification(conn: &PgConnection, data: &OfferToUpdate, new_price: &Price) {
    for product in &data.products {
        // Get users who are notified about this product from database
        let db_response =
            database::models::product::queries::users_notified_of_product(conn, product.id);

        // Handle db_response
        let user_emails = match db_response {
            Ok(v) => v
                .into_iter()
                .map(|user| user.email)
                .collect::<Vec<String>>(),
            Err(e) => {
                error!(
                    "Couldn't get users who are notified about {:?}. Error: {:?}",
                    data.offer, e
                );
                return;
            }
        };

        // Email them
        email_many(
            &product.name,
            &data.offer.url,
            &data.last_price,
            new_price,
            &user_emails,
        );
    }
}
