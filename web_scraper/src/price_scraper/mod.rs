use crate::downloaders::fantoccini::FantocciniDownloader;
use crate::downloaders::reqwest::ReqwestDownloader;
use crate::{config::PriceScraperConfig, downloaders::Downloader};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

///////////////////////////////////////////////////////////////////////////////
// Private Modules

mod utils;

///////////////////////////////////////////////////////////////////////////////
// Public Errors

#[derive(thiserror::Error, Debug)]
#[error("Cannot get price from the page")]
pub enum GetPriceError {
    PageNotSupported,
    PriceNotFound,
    ErrorDownloadingPage,
    PageDownloadTimeout,
    Redirected,
}

#[derive(thiserror::Error, Debug)]
#[error("Cannot get potential prices from the page")]
pub enum GetPotentialPricesError {
    #[error("Page is not supported")]
    PageNotSupported,
    #[error("Redirected")]
    Redirected,
    #[error("Cannot parse the css selector")]
    CannotParseCssSelector,
    #[error("Cannot download page")]
    CannotDownloadPage,
    #[error("Other error")]
    Other,
}

///////////////////////////////////////////////////////////////////////////////
// Private Errors

#[derive(thiserror::Error, Debug)]
#[error("Selector not found for this page. Probably it's not supported.")]
struct GetSelectorError {}

///////////////////////////////////////////////////////////////////////////////
// Structs

pub struct PriceScraper {
    reqwest_selectors: HashMap<String, String>,
    fantoccini_selectors: HashMap<String, String>,
    pub reqwest_client: reqwest::Client,
}

impl PriceScraper {
    /////////////////////////////////////////////////////////////////////////////////////////////////////////
    // PUBLIC

    pub async fn new(config: PriceScraperConfig) -> Self {
        let reqwest_client = reqwest::ClientBuilder::new()
            .user_agent(&config.user_agent)
            // TODO: timeout from config
            .timeout(std::time::Duration::from_secs(20))
            .build()
            .unwrap_or_else(|e| {
                panic!(
                    "Error while trying building a reqwest client: {}",
                    e.to_string()
                );
            });

        Self {
            reqwest_selectors: config.reqwest_selectors,
            fantoccini_selectors: config.fantoccini_selectors,
            reqwest_client,
        }
    }

    /// This is the main function you want to use.
    pub async fn get_price(
        &self,
        url: &str,
        last_available_price: Option<f64>,
    ) -> error_stack::Result<f64, GetPriceError> {
        // TODO: First sleep time in the some config.json
        // Exponential sleep duration. 10 secs
        let mut duration: u64 = 10;

        // TODO: Move this into config
        let fairness_tries = 3;

        // Get price, first try
        let mut price = self.get_price_retry_error(url).await?;

        // Loop if the price seems not fair, suspicious
        for _ in 0..fairness_tries {
            let last_price = match last_available_price {
                Some(v) => v,
                // If there's no last available price return gotten price early
                None => return Ok(price),
            };

            let percent_difference = ((price / last_price) - 1.0).abs();

            // TODO: Condition in some config
            // If difference in price is not so much different then break
            if percent_difference < 0.10 {
                break;
            }

            // But in the other case, price might be suspiciously different

            // Sleep for `duration` (secs) time
            sleep(Duration::from_secs(duration)).await;
            // TODO: Multiplier in the some config.json
            duration = duration * 2;

            // Try to get price again
            price = self.get_price_retry_error(url).await?;
        }

        return Ok(price);
    }

    /// Maybe you want some debugging info, so you can get blocks on the page, that bot thinks there are prices in
    pub async fn get_potential_prices_blocks(
        &self,
        url: &str,
    ) -> error_stack::Result<Vec<String>, GetPotentialPricesError> {
        // Get downloader function and css selector
        let (downloader, css_selector) = self
            .get_downloader_and_css_selector(url)
            .map_err(|error| {
                error
                    .change_context(GetPotentialPricesError::PageNotSupported)
                    .attach_printable(format!("Cannot get css selector for this url from config file, probably this domain is not supported. Url: {}", url))
            })?;

        // Dowload page
        let page = downloader
            .download_page(&self, url)
            .await
            .map_err(|error| {
                error_stack::report!(error)
                    .change_context(GetPotentialPricesError::CannotDownloadPage)
                    .attach_printable("Cannot download the page")
                    .attach_printable(format!("Url: {}", url))
            })?;

        // Construct html scraper
        let scraper_selector = scraper::Selector::parse(css_selector).map_err(|error| {
            error_stack::report!(GetPotentialPricesError::CannotParseCssSelector)
                .attach_printable(format!(
                    "Couldn't parse css selector. Given css selector: {}",
                    css_selector
                ))
                .attach_printable(format!("Cause: {:?}", error))
        })?;

        // Parse document
        let source = scraper::Html::parse_document(&page);

        // Find potential matches
        let matches = source.select(&scraper_selector);

        // Convert matches to vec of strings
        let matches = matches
            .map(|el| el.text().collect::<String>())
            .collect::<Vec<String>>();

        // Return matches
        Ok(matches)
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////////////
    // PRIVATE

    async fn get_price_retry_error(&self, url: &str) -> error_stack::Result<f64, GetPriceError> {
        // TODO: First sleep time in the some config.json
        // Exponential sleep duration. 10 secs
        let mut duration: u64 = 10;

        // TODO: Move this into config
        let number_of_tries = 3;

        // Get price, first try
        let mut price = self.get_price_once(url).await;

        match price {
            Ok(v) => return Ok(v),
            Err(_) => {}
        }

        // Loop if the price seems not fair, suspicious
        for _ in 0..number_of_tries {
            // Try to get price
            price = self.get_price_once(url).await;

            if price.is_ok() {
                break;
            }

            // But in the other case, price might be suspiciously different
            // Sleep for `duration` (secs) time
            sleep(Duration::from_secs(duration)).await;
            // TODO: Multiplier in the some config.json
            duration = duration * 2;
        }

        return price;
    }

    async fn get_price_once(&self, url: &str) -> error_stack::Result<f64, GetPriceError> {
        Ok(self
            .get_potential_prices_blocks(url)
            .await
            .map_err(|error| error.change_context(GetPriceError::ErrorDownloadingPage))?
            .iter()
            .map(|s| utils::string_to_float(s))
            .flatten()
            .next()
            .ok_or_else(|| GetPriceError::PriceNotFound)?)
    }

    fn get_downloader_and_css_selector<'a>(
        &'a self,
        url: &str,
    ) -> error_stack::Result<(Box<dyn Downloader>, &'a str), GetSelectorError> {
        // First look for selector in reqwest_selectors
        let reqwest_selector = self
            .reqwest_selectors
            .iter()
            .find(|(k, _)| url.contains(*k));

        if let Some((_, v)) = reqwest_selector {
            return Ok((Box::new(ReqwestDownloader), v));
        }

        // Then look for selector in fantoccini_selectors
        let fantoccini_selector = self
            .fantoccini_selectors
            .iter()
            .find(|(k, _)| url.contains(*k));

        if let Some((_, v)) = fantoccini_selector {
            return Ok((Box::new(FantocciniDownloader), v));
        }

        // If not found, this means page is not supported
        return Err(
            error_stack::report!(GetSelectorError {}).attach_printable(format!(
                "Page with domain of that url was not found in web_scraper_settings.json. Url: {}",
                url
            )),
        );
    }
}
