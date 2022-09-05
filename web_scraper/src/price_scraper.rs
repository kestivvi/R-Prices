use crate::downloaders::fantoccini::FantocciniDownloader;
use crate::downloaders::reqwest::ReqwestDownloader;
use crate::downloaders::DownloadingError;
use crate::{config::PriceScraperConfig, downloaders::Downloader};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
pub enum Error {
    ErrorDownloadingPage,
    PageNotSupported,
    ErrorCreatingFantocciniClient,
}

#[derive(Debug)]
pub enum PriceEnum {
    Available(f64),
    TemporarilyUnavailable,
    Unavailable,
    PriceNotFound,
    SiteNotFound,
}

pub struct PriceScraper {
    reqwest_selectors: HashMap<String, String>,
    fantoccini_selectors: HashMap<String, String>,
    pub reqwest_client: reqwest::Client,
}

#[derive(thiserror::Error, Debug)]
#[error("Cannot get price from the page")]
pub enum GetPriceError {
    PageNotSupported,
    PriceNotFound,
    ErrorDownloadingPage,
    PageDownloadTimeout,
    Redirected,
}

pub enum RequestType {
    Reqwest,
    Fantoccini,
}

#[derive(thiserror::Error, Debug)]
#[error("Couldn't get price from the source of page")]
enum GetPriceFromSourceError {
    CannotParseCssSelector,
    NoMatchesForPricesFound,
    CannotParseFirstMatchToPrice,
}

#[derive(thiserror::Error, Debug)]
#[error("Couldn't parse string to float")]
struct FloatParseError;

#[derive(thiserror::Error, Debug)]
#[error("Selector not found for this page. Probably it's not supported.")]
pub struct GetSelectorError {}

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

    pub async fn get_price(&self, url: &str) -> error_stack::Result<f64, GetPriceError> {
        let (downloader, css_selector) = self
            .get_download_page_fn_and_css_selector(url)
            .map_err(|error| {
                error
                    .change_context(GetPriceError::PageNotSupported)
                    .attach_printable(format!("Cannot get css selector for this url from config file, probably this domain is not supported. Url: {}", url))
            })?;

        let page = downloader
            .download_page(&self, url)
            .await
            .map_err(|error| match error.current_context() {
                DownloadingError::Redirection => {
                    error
                    .change_context(GetPriceError::Redirected)
                    .attach_printable("Couldn't download page because server redirected away from it. Probably it doesn't exist")
                    .attach_printable(format!("Requested url: {}", url))
                },
                DownloadingError::NotValidInputUrl => todo!(),
                _ => error
                    .change_context(GetPriceError::ErrorDownloadingPage)
                    .attach_printable("Cannot download page for some reason")
                    .attach_printable(format!("Requested url: {}", url))
            })?;

        let price = Self::get_price_from_html(&page, &css_selector).map_err(|error| {
            error
                .change_context(GetPriceError::PriceNotFound)
                .attach_printable("Error trying to get price from page")
                .attach_printable(format!("Url: {}", url))
        })?;

        Ok(price)
    }

    pub async fn get_price_multiple_tries(
        &self,
        url: &str,
        number_of_tries: u64,
    ) -> error_stack::Result<f64, GetPriceError> {
        let mut i = 0;
        let mut dur = (i + 1) * 10;
        let price = loop {
            match self.get_price(url).await {
                Ok(v) => break Ok(v),
                Err(error) => {
                    if i < number_of_tries {
                        i += 1;
                        sleep(Duration::from_secs(dur)).await;
                        dur = dur * 2;
                        continue;
                    }
                    break Err(error);
                }
            }
        };
        price
    }

    pub async fn get_price_multiple_tries_and_fairness(
        &self,
        url: &str,
        download_tries: u64,
        fairness_tries: u64,
        last_available_price: Option<f64>,
    ) -> error_stack::Result<f64, GetPriceError> {
        let mut i = 0;
        let mut dur = (i + 1) * 10;
        let price = loop {
            match self.get_price_multiple_tries(url, download_tries).await {
                Ok(v) => {
                    if let Some(last_available_price) = last_available_price {
                        let percent_difference = ((v / last_available_price) - 1.0).abs();
                        if percent_difference > 0.10 && i < fairness_tries {
                            i += 1;
                            sleep(Duration::from_secs(dur)).await;
                            dur = dur * 2;
                            continue;
                        }
                    }
                    break Ok(v);
                }
                Err(error) => break Err(error),
            }
        };
        price
    }

    fn get_download_page_fn_and_css_selector<'a>(
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

    /////////////////////////////////////////////////////////////////////////////////////////////////////////
    // PRIVATE

    fn string_to_float(s: &str) -> error_stack::Result<f64, FloatParseError> {
        let mut first_dot = true;
        s.chars()
            .into_iter()
            .map(|c| if c == ',' { '.' } else { c })
            // TODO: Try doing it with Iterator::scan()
            .filter(|&c| {
                let first_occurence_of_dot = c == '.' && first_dot;
                if first_occurence_of_dot {
                    first_dot = false;
                }
                c.is_numeric() || first_occurence_of_dot
            })
            .collect::<String>()
            .parse::<f64>()
            .map_err(|error| {
                error_stack::report!(error)
                    .change_context(FloatParseError)
                    .attach_printable(format!("Cannot parse string to float. String: {}", s))
            })
    }

    fn get_price_from_html(
        document: &str,
        css_selector: &str,
    ) -> error_stack::Result<f64, GetPriceFromSourceError> {
        // Prepare for parsing
        // Get selector struct
        let scraper_selector = scraper::Selector::parse(css_selector).map_err(|error| {
            error_stack::report!(GetPriceFromSourceError::CannotParseCssSelector)
                .attach_printable(format!(
                    "Couldn't parse css selector. Given css selector: {}",
                    css_selector
                ))
                .attach_printable(format!("Cause: {:?}", error))
        })?;

        // Parse document
        let source = scraper::Html::parse_document(&document);

        // Find potential matches
        let matches = source.select(&scraper_selector);

        // Convert matches to vec of strings
        let matches = matches
            .map(|el| el.text().collect::<String>())
            .collect::<Vec<String>>();

        // Handle no matches case
        if matches.len() == 0 {
            return Err(
                error_stack::report!(GetPriceFromSourceError::NoMatchesForPricesFound)
                    .attach_printable("No matches for potential matches found")
                    .attach(css_selector.to_owned())
                    .attach(document.to_owned()),
            );
        }
        // Convert to prices
        let potential_prices = matches
            .iter()
            .map(|el| Self::string_to_float(&el))
            .collect::<Vec<_>>();

        // Get first price
        let price = potential_prices
            .into_iter()
            .next()
            .ok_or_else(|| {
                error_stack::report!(GetPriceFromSourceError::NoMatchesForPricesFound)
                    .attach_printable("This code should be unreacheable, but maybe I'm wrong")
                    .attach_printable("No matches for potential matches found")
                    .attach(css_selector.to_owned())
                    .attach(document.to_owned())
            })?
            .map_err(|error| {
                error
                    .change_context(GetPriceFromSourceError::CannotParseFirstMatchToPrice)
                    .attach_printable("Couldn't parse first match to price")
                    .attach(matches)
                    .attach(css_selector.to_owned())
                    .attach(document.to_owned())
            })?;

        Ok(price)
    }
}
