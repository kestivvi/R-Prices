use database::db::get_pool;
use log::info;
use std::time::Duration;
use web_scraper::config::PriceScraperConfig;
use web_scraper::price_scraper::PriceScraper;
use web_scraper::tasks::update_all_offers_and_send_notifications;
use web_scraper::utils::init_env_and_logging;

///////////////////////////////////////////////////////////////////////////////
// Run function

async fn run() {
    loop {
        // Get configuration and other stuff
        info!("Getting configuration");
        let price_scraper_config = PriceScraperConfig::default();

        {
            // Get things
            let pool = get_pool(&price_scraper_config.database_url);
            let conn = &pool.get().unwrap();
            let scraper = PriceScraper::new(price_scraper_config.clone()).await;

            // Run things
            info!("Updating products");

            let timer = std::time::Instant::now();
            update_all_offers_and_send_notifications(&scraper, conn).await;
            let elapsed_time = timer.elapsed().as_secs_f32();
            info!("Updating prices took {} secs", elapsed_time);
        }

        // Break or sleep
        if price_scraper_config.run_in_loop == false {
            break;
        } else {
            info!("Sleeping {} seconds", price_scraper_config.interval);
            tokio::time::sleep(Duration::from_secs(price_scraper_config.interval)).await;
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Main

#[tokio::main()]
async fn main() {
    info!("Initializing environment variables and logging!");
    init_env_and_logging();
    run().await;
}
