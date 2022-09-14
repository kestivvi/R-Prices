use fantoccini::ClientBuilder;

use crate::price_scraper::PriceScraper;

use super::{get_url_struct, Downloader, DownloadingError};

pub struct FantocciniDownloader;

#[async_trait::async_trait]
impl Downloader for FantocciniDownloader {
    async fn download_page(
        &self,
        _price_scraper: &PriceScraper,
        url: &str,
    ) -> error_stack::Result<String, DownloadingError> {
        // Check if url is valid
        let url_struct = get_url_struct(url).map_err(|error| {
            error_stack::report!(error)
                .change_context(DownloadingError::NotValidInputUrl)
                .attach_printable(format!("Given url is not valid. Url: {}", url))
        })?;

        // Create fantoccini client
        let fantoccini_client = create_fantoccini_client().await.map_err(|error| {
            error_stack::report!(error)
                .change_context(DownloadingError::CreatingDownloaderClient)
                .attach_printable("Couldn't create fantoccini client")
        })?;

        // Go to the page
        fantoccini_client.goto(url).await.map_err(|error| {
            error_stack::report!(error)
                .change_context(DownloadingError::Other)
                .attach_printable(format!(
                    "Couldn't go to the given url with the fantoccini client. Url: {}",
                    url
                ))
        })?;

        // Get url of the page where we landed
        let downloaded_url = fantoccini_client
            .current_url()
            .await
            .map_err(|error| {
                error_stack::report!(error)
                    .change_context(DownloadingError::CannotGetDownloadedUrl)
                    .attach_printable(format!("Couldn't get url from the response of fantoccini client. Requested url was: {}", url))
            })?;

        // Handle redirection case
        if url_struct != downloaded_url {
            return Err(error_stack::report!(DownloadingError::Redirection)
                .attach_printable("Downloaded page comes from different url than requested.")
                .attach_printable(format!("Requested url: {}", url))
                .attach_printable(format!("Downloaded url: {}", downloaded_url)));
        }

        // Get html source
        let document = fantoccini_client.source().await.map_err(|error| {
            error_stack::report!(error)
                .change_context(DownloadingError::GetSourceFromResponse)
                .attach_printable(format!(
                    "Couldn't get the source html code from response of the given url. Url: {}",
                    url
                ))
        })?;

        Ok(document)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Couldn't create fantoccini client")]
enum CreateFantocciniError {
    BadWebdriverUrl,
    ConnectingToWebdriver,
    SettingUserAgent,
}

async fn create_fantoccini_client() -> error_stack::Result<fantoccini::Client, CreateFantocciniError>
{
    // TODO: Get url for webdriver from config
    let webdriver_url = "http://localhost:4444";

    // Connecting to webdriver
    let fantoccini_client = ClientBuilder::native()
        .connect(webdriver_url)
        .await
        .map_err(|error| {
            match error {
                fantoccini::error::NewSessionError::BadWebdriverUrl(error) => {
                    error_stack::report!(error)
                    .change_context(CreateFantocciniError::BadWebdriverUrl)
                    .attach_printable(
                        format!(
                            "Url provided for connecting to selenium webdriver for fantoccini is wrong. 
                            Please check config file and if docker selenium webdriver is working properly
                            Provided webdriver url: {}", webdriver_url
                        )
                    )
                },
                error => {
                    error_stack::report!(error)
                    .change_context(CreateFantocciniError::ConnectingToWebdriver)
                    .attach_printable(
                        format!(
                            "Error occured while connecting to selenium webdriver. 
                            Please check config file and if docker selenium webdriver is working properly. 
                            Maybe webdriver is overloaded or misconfigured.
                            Maybe it needs restarting.
                            Provided webdriver url: {}", webdriver_url
                        )
                    )
                }
            }
        })?;

    // Set the user-agent
    // TODO: Get random user-agent from some list in the config
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36";
    fantoccini_client
        .set_ua(user_agent)
        .await
        .map_err(|error| {
            error_stack::report!(error)
                .change_context(CreateFantocciniError::SettingUserAgent)
                .attach_printable(format!(
                    "Couldn't set user agent for some unknown reason. Tried to set user agent: {}",
                    user_agent
                ))
        })?;

    Ok(fantoccini_client)
}
