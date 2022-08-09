use database;
use database::db::get_pool;
use database::models::offer::queries::offers_of_product;
use database::models::offer::Offer;
use database::models::price::queries::get_last_prices_of_offer;
use database::models::price::Availability;
use database::models::price::CreatePriceInput;
use database::models::price::Price;
use database::models::product::queries::products_with_notification;
use database::models::product::Product;
use diesel::PgConnection;
use env_logger::Env;
use log::debug;
use log::error;
use log::info;
use std::time::Duration;
use web_scraper::config::PriceScraperConfig;
use web_scraper::email::email_many;
use web_scraper::price_scraper::Error;
use web_scraper::price_scraper::PriceEnum;
use web_scraper::price_scraper::PriceScraper;

fn on_price_change(
    conn: &PgConnection,
    product: &Product,
    offer: &Offer,
    old_price: &Price,
    new_price: &Price,
) {
    // debug!("Price for {offer:?} has changed from {old_price:?} to {new_price:?}");

    let res = database::models::product::queries::users_notified_of_product(conn, product.id);

    let user_emails = match res {
        Ok(v) => v
            .into_iter()
            .map(|user| user.email)
            .collect::<Vec<String>>(),
        Err(e) => {
            error!(
                "Couldn't get users who are notified about {:?}. Error: {:?}",
                offer, e
            );
            return;
        }
    };

    // Make sure it has notification turned on
    // if product.notification {
    email_many(
        &product.name,
        &offer.url,
        old_price,
        new_price,
        &user_emails,
    );
    // }
}

fn find_price_changes(conn: &PgConnection) {
    let products = match products_with_notification(conn) {
        Ok(v) => v,
        Err(e) => {
            error!("Could not get products from database: {:?}", e);
            return;
        }
    };

    for product in products {
        let offers = match offers_of_product(conn, product.id) {
            Ok(v) => v,
            Err(e) => {
                error!(
                    "Could not get offers from database for the product: {:?}",
                    product
                );
                error!("Raw error: {:?}", e);
                continue;
            }
        };

        for offer in offers {
            let prices = match get_last_prices_of_offer(conn, offer.id, 2) {
                Ok(v) => v,
                Err(e) => {
                    error!(
                        "Could not get prices from database for the offer: {:?}",
                        offer
                    );
                    error!("Raw error: {:?}", e);
                    continue;
                }
            };

            if prices.len() < 2 || prices[0].value == prices[1].value {
                continue;
            }

            on_price_change(conn, &product, &offer, &prices[1], &prices[0]);
        }
    }
}

async fn update_all_products_once(scraper: &PriceScraper, conn: &PgConnection) {
    // Get all offers in database
    let offers = database::models::offer::queries::all_offers(&conn).unwrap();
    let mut handles = Vec::new();

    // For each offer get its price and insert them to database
    for offer in offers {
        // Get price the price from the website
        // TODO: If price not found, update db with price and field availiability
        let task = async move {
            let mut debug_log = format!("\n- Trying to update price of {}", offer.url);

            let price_enum = match scraper.get_price_from_url(&offer.url).await {
                Ok(v) => {
                    debug_log.push_str(&format!("\n- Price found: {:?}", v));
                    v
                }
                Err(e) => match e {
                    Error::ErrorDownloadingPage => {
                        debug!("{}", debug_log);
                        error!("Error downloading page: {}", offer.url);
                        return;
                    }
                    Error::PageNotSupported => {
                        debug!("{}", debug_log);
                        error!("Page is not supported yet: {}", offer.url);
                        return;
                    }
                    Error::ErrorCreatingFantocciniClient => {
                        debug!("{}", debug_log);
                        error!("Error Creating Fantoccini Client to get: {}", offer.url);
                        return;
                    }
                },
            };

            let mut price = None;
            let mut product_availability = Availability::Available;

            match price_enum {
                PriceEnum::Available(v) => price = Some(v),
                PriceEnum::TemporarilyUnavailable => {
                    product_availability = Availability::TemporarilyUnavailable
                }
                PriceEnum::Unavailable => product_availability = Availability::Unavailable,
                PriceEnum::PriceNotFound => product_availability = Availability::PriceNotFound,
                PriceEnum::SiteNotFound => product_availability = Availability::SiteNotFound,
            }

            // Insert price to database
            let db_response = database::models::price::mutations::create_price(
                conn,
                CreatePriceInput {
                    offer_id: offer.id,
                    value: price,
                    availability: product_availability,
                },
            );

            // Handle possible error from database
            match db_response {
                Ok(_) => (),
                Err(e) => {
                    debug!("{}", debug_log);
                    error!(
                        "Error inserting updated price to database for: {:?}\nRaw error: {:?}",
                        offer, e
                    );
                    return;
                }
            };
            debug_log.push_str(&format!("\n- Price succesfully updated in database"));
            debug!("{}", debug_log);
        };

        handles.push(task);
    }

    futures::future::join_all(handles).await;
}

///////////////////////////////////////////////////////////////////////////////
// Utils

/// Loads env variables from .env and initializes env_logger
fn init_env_and_logging() {
    dotenv::dotenv().ok();
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
}

///////////////////////////////////////////////////////////////////////////////
// Run function

async fn run() {
    loop {
        // Get configuration and other stuff
        debug!("Getting configuration");
        let price_scraper_config = PriceScraperConfig::new();

        {
            let pool = get_pool(&price_scraper_config.database_url);
            let conn = &pool.get().unwrap();
            let scraper = PriceScraper::new(price_scraper_config.clone()).await;

            // Run things
            debug!("Updating products");

            let timer = std::time::Instant::now();
            update_all_products_once(&scraper, conn).await;
            let elapsed_time = timer.elapsed().as_secs_f32();
            info!("Updating prices took {} secs", elapsed_time);

            debug!("Looking for changes");
            let timer = std::time::Instant::now();
            find_price_changes(conn);
            let elapsed_time = timer.elapsed().as_secs_f32();
            info!(
                "Looking for changes and emailing took {} secs",
                elapsed_time
            );
        }

        // Break or sleep
        if price_scraper_config.run_in_loop == false {
            break;
        } else {
            info!("Sleeping {} seconds", price_scraper_config.interval);
            tokio::time::sleep(Duration::from_secs(price_scraper_config.interval)).await;
            // std::thread::sleep(Duration::from_secs(price_scraper_config.interval));
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Main

#[tokio::main(flavor = "current_thread")]
async fn main() {
    info!("Starting up!");

    init_env_and_logging();

    info!("Hello, I'm a web_scraper");
    info!("I scrub through database and update prices");
    info!("I email notifications about changing prices too");
    run().await;
}
