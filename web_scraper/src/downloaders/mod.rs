use crate::price_scraper::PriceScraper;

pub mod fantoccini;
pub mod reqwest;

#[derive(thiserror::Error, Debug)]
#[error("Couldn't download page")]
pub enum DownloadingError {
    Timeout,
    Redirection,
    CreatingDownloaderClient,
    GetSourceFromResponse,
    CannotGetDownloadedUrl,
    NotValidInputUrl,
    Other,
}

#[async_trait::async_trait]
pub trait Downloader {
    async fn download_page(
        &self,
        price_scraper: &PriceScraper,
        url: &str,
    ) -> error_stack::Result<String, DownloadingError>;
}

fn get_url_struct(url: &str) -> Result<url::Url, url::ParseError> {
    url::Url::parse(url)
}
