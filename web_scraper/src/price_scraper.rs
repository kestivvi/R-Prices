use crate::config::PriceScraperConfig;
use fantoccini::ClientBuilder;
use std::{collections::HashMap, time::Duration};

#[derive(Debug)]
pub enum Error {
    ErrorDownloadingPage,
    PageNotSupported,
    ErrorCreatingFantocciniClient,
}

enum Selector {
    Reqwest(String),
    Fantoccini(String),
    NotFound,
}

#[derive(Debug)]
pub enum PriceEnum {
    Available(f64),
    TemporarilyUnavailable,
    Unavailable,
    PriceNotFound,
    SiteNotFound,
}

struct HtmlDocument {
    document: String,
    url: String,
}

pub struct PriceScraper {
    reqwest_selectors: HashMap<String, String>,
    fantoccini_selectors: HashMap<String, String>,
    reqwest_client: reqwest::Client,
}

impl PriceScraper {
    /////////////////////////////////////////////////////////////////////////////////////////////////////////
    // PUBLIC

    pub async fn new(config: PriceScraperConfig) -> Self {
        let reqwest_client = reqwest::ClientBuilder::new()
            .user_agent(&config.user_agent)
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

    pub async fn get_price_from_url(&self, url: &str) -> Result<PriceEnum, Error> {
        let selector = self.get_selector(url);
        let mut document = String::default();

        for i in 1..=4 {
            match self.get_html(url, &selector).await {
                Ok(v) => {
                    let gotten_url;
                    HtmlDocument {
                        document,
                        url: gotten_url,
                    } = v;
                    if !Self::are_urls_equal(url, &gotten_url) {
                        return Ok(PriceEnum::SiteNotFound);
                    }
                    break;
                }
                Err(e) => match e {
                    Error::ErrorDownloadingPage => {
                        log::error!("Cannot download page: {url}");

                        if i == 4 {
                            return Err(Error::ErrorDownloadingPage);
                        }

                        log::error!("Trying again in a {} seconds for the {}/3 time", i * 10, i);
                        tokio::time::sleep(Duration::from_secs(i * 10)).await;
                        log::error!("Trying now");
                    }
                    err => return Err(err),
                },
            };
        }
        let selector = match selector {
            Selector::Reqwest(v) => v,
            Selector::Fantoccini(v) => v,
            Selector::NotFound => return Err(Error::PageNotSupported),
        };

        match self.get_price_from_html(&document, &selector) {
            Some(v) => Ok(PriceEnum::Available(v)),
            None => Ok(PriceEnum::PriceNotFound),
        }
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////////////
    // PRIVATE

    fn are_urls_equal(url_1: &str, url_2: &str) -> bool {
        let it1 = url_1.chars().filter(|&c| c.is_ascii_alphanumeric());
        let it2 = url_2.chars().filter(|&c| c.is_ascii_alphanumeric());
        it1.eq(it2)
    }

    fn get_selector(&self, url: &str) -> Selector {
        let found = self
            .reqwest_selectors
            .iter()
            .find(|(k, _)| url.contains(*k));

        if let Some((_, v)) = found {
            return Selector::Reqwest(v.to_owned());
        }

        let found = self
            .fantoccini_selectors
            .iter()
            .find(|(k, _)| url.contains(*k));

        if let Some((_, v)) = found {
            return Selector::Fantoccini(v.to_owned());
        }

        Selector::NotFound
    }

    async fn get_html(&self, url: &str, selector: &Selector) -> Result<HtmlDocument, Error> {
        // TODO: Get url of the downloaded website
        // self.reqwest_client.get(url).send().await.unwrap().url()
        match selector {
            Selector::Reqwest(_) => {
                let response_result = self.reqwest_client.get(url).send().await;
                match response_result {
                    Ok(response) => {
                        let url = response.url().to_string();
                        let document_result = response.text().await;
                        match document_result {
                            Ok(document) => Ok(HtmlDocument {
                                document: document.to_owned(),
                                url,
                            }),
                            Err(_) => return Err(Error::ErrorDownloadingPage),
                        }
                    }
                    Err(_) => return Err(Error::ErrorDownloadingPage),
                }
            }
            Selector::Fantoccini(_) => {
                let fantoccini_client = create_fantoccini_client().await;

                fantoccini_client
                    .goto(url)
                    .await
                    .map_err(|_| Error::ErrorDownloadingPage)?;

                let document = fantoccini_client
                    .source()
                    .await
                    .map_err(|_| Error::ErrorDownloadingPage)?;

                let url = fantoccini_client
                    .current_url()
                    .await
                    .map_err(|_| Error::ErrorDownloadingPage)?
                    .to_string();

                Ok(HtmlDocument { document, url })
            }
            Selector::NotFound => Err(Error::PageNotSupported),
        }
    }

    fn get_price_from_html(&self, document: &str, selector: &str) -> Option<f64> {
        fn string_to_float(s: &str) -> Option<f64> {
            let mut first_dot = true;
            s.chars()
                .into_iter()
                .map(|c| if c == ',' { '.' } else { c })
                .filter(|&c| {
                    let first_occurence_of_dot = c == '.' && first_dot;
                    if first_occurence_of_dot {
                        first_dot = false;
                    }
                    c.is_numeric() || first_occurence_of_dot
                })
                .collect::<String>()
                .parse::<f64>()
                .ok()
        }

        // Prepare for parsing
        let scraper_selector = scraper::Selector::parse(selector).unwrap();
        let html = scraper::Html::parse_document(&document);

        // Find potential matches
        let potential_prices = html.select(&scraper_selector);

        let potential_prices = potential_prices
            .map(|el| el.text())
            .map(|el| el.collect::<String>())
            .collect::<Vec<String>>();

        let potential_prices = potential_prices
            .iter()
            .map(|el| string_to_float(&el))
            .flatten()
            .collect::<Vec<f64>>();

        // Get first price if it exists at all
        potential_prices.into_iter().next()
    }
}

async fn create_fantoccini_client() -> fantoccini::Client {
    let mut i = 1;
    loop {
        let fantoccini_client = ClientBuilder::native()
            .connect("http://localhost:4444")
            .await;

        let fantoccini_client = match fantoccini_client {
            Ok(v) => v,
            Err(e) => {
                if i == 4 {
                    panic!(
                        "Error while trying building a fantoccini client: {}",
                        e.to_string()
                    );
                }
                log::error!(
                    "Can't create fantoccini client, trying again in a {} seconds for the {} time",
                    i * 10,
                    i
                );
                i += 1;
                tokio::time::sleep(Duration::from_secs(i * 10)).await;
                continue;
            }
        };

        let setting_ua_result = fantoccini_client.set_ua(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36"
            )
            .await;

        match setting_ua_result {
            Ok(v) => v,
            Err(e) => {
                if i == 4 {
                    panic!(
                        "Error while trying building a fantoccini client: {}",
                        e.to_string()
                    );
                }
                log::error!(
                    "Can't set fantoccini client user-agent, trying creating entire client again in a {} seconds for the {} time",
                    i * 10,
                    i
                );
                i += 1;
                tokio::time::sleep(Duration::from_secs(i * 10)).await;
                continue;
            }
        };

        break fantoccini_client;
    }
}
