use super::{get_url_struct, Downloader, DownloadingError};
use crate::price_scraper::PriceScraper;

pub struct ReqwestDownloader;

#[async_trait::async_trait]
impl Downloader for ReqwestDownloader {
    async fn download_page(
        &self,
        price_scraper: &PriceScraper,
        url: &str,
    ) -> error_stack::Result<String, DownloadingError> {
        // Check if url is valid
        let url_struct = get_url_struct(url).map_err(|error| {
            error_stack::report!(error)
                .change_context(DownloadingError::NotValidInputUrl)
                .attach_printable("Given url is not valid")
                .attach_printable(format!("Requested url: {}", url))
        })?;

        // Send request and get a response
        let response = price_scraper
            .reqwest_client
            .get(url)
            .send()
            .await
            .map_err(|error| {
                if error.is_timeout() {
                    error_stack::report!(error)
                        .change_context(DownloadingError::Timeout)
                        .attach_printable("Cannot download page due to timeout")
                        .attach_printable(format!("Requested url: {}", url))
                } else if error.is_builder() {
                    error_stack::report!(error)
                        .change_context(DownloadingError::CreatingDownloaderClient)
                        .attach_printable(
                            "Cannot download page cause of error building reqwest client",
                        )
                        .attach_printable(format!("Requested url: {}", url))
                } else if error.is_redirect() {
                    error_stack::report!(error)
                        .change_context(DownloadingError::Redirection)
                        .attach_printable("Cannot download page cause of redirection error")
                        .attach_printable(format!("Requested url: {}", url))
                } else {
                    error_stack::report!(error)
                        .change_context(DownloadingError::Other)
                        .attach_printable("Cannot download page cause of unknown error")
                        .attach_printable(format!("Requested url: {}", url))
                }
            })?;

        // Get url of the downloaded page
        let downloaded_url = response.url();

        // Handle redirection case
        if url_struct != *downloaded_url {
            return Err(error_stack::report!(DownloadingError::Redirection)
                .attach_printable("Downloaded page comes from different url than requested")
                .attach_printable(format!("Requested url: {}", url))
                .attach_printable(format!("Downloaded url: {}", downloaded_url)));
        }

        // Get the source html
        let text = response.text().await.map_err(|error| {
            error_stack::report!(error)
                .change_context(DownloadingError::GetSourceFromResponse)
                .attach_printable("Couldn't retrieve text from downloaded page for unknown reason")
                .attach_printable(format!("Requested url: {}", url))
        })?;

        Ok(text)
    }
}
