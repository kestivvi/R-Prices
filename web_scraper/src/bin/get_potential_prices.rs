use web_scraper::config::PriceScraperConfig;
use web_scraper::price_scraper::PriceScraper;
use web_scraper::utils::init_env_and_logging;

#[tokio::main()]
async fn main() {
    init_env_and_logging();

    let url: String = match std::env::args().skip(1).take(1).next() {
        Some(v) => v,
        None => {
            eprintln!("Required first positional argument => page_url: string");
            return;
        }
    };

    let config = PriceScraperConfig::default();
    let scraper = PriceScraper::new(config).await;

    let potential_prices_result = scraper.get_potential_prices_blocks(&url).await;

    let potential_prices = match potential_prices_result {
        Ok(v) => v,
        Err(err) => {
            eprintln!("Error occured");
            eprintln!("{:?}", err);
            return;
        }
    };

    if potential_prices.is_empty() {
        println!("Not found any potential blocks with price");
        return;
    }

    println!("Potential blocks with price:");
    for (i, block) in potential_prices.into_iter().enumerate() {
        println!("{}. <{}>", i, block);
    }
}
