use std::io::{Cursor, Read};

use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use zed_extension_api::{
    self as zed,
    http_client::{HttpMethod::*, RedirectPolicy::*},
};
use zip::ZipArchive;

pub struct PublicGalleryClient {
    base_url: String,
    api_version: String,
}

impl PublicGalleryClient {
    /// Creates a new API client for the Visual Studio extension marketplace.
    pub fn new() -> Self {
        Self {
            base_url: "https://marketplace.visualstudio.com/".into(),
            api_version: "7.2-preview.1".into(),
        }
    }

    /// Downloads a package from the Visual Studio extension marketplace.
    ///
    /// Returns the raw bytes of the downloaded .vsix package.
    fn download_package_as_raw_bytes(
        &self,
        author: &str,
        name: &str,
        version: &str,
    ) -> Result<Vec<u8>, String> {
        let request = zed::http_client::HttpRequest::builder()
            .method(Get)
            .url(format!(
                "{}/_apis/public/gallery/publishers/{}/vsextensions/{}/{}/vspackage",
                self.base_url, author, name, version
            ))
            .header(
                "accept",
                format!("application/vsix; api-version={}", self.api_version),
            )
            .header("accept-encoding", "gzip, deflate")
            .header("user-agent", "Zed extension for Haxe v0.1")
            .redirect_policy(NoFollow)
            .build()
            .unwrap();

        let response = zed::http_client::fetch(&request)?;
        Ok(response.body)
    }

    /// Downloads a package from the Visual Studio extension marketplace.
    pub fn download_package_as_zip(
        &self,
        author: &str,
        name: &str,
        version: &str,
    ) -> Result<ZipArchive<Cursor<Vec<u8>>>, String> {
        let raw_bytes = self.download_package_as_raw_bytes(author, name, version)?;

        // The .vsix package is gzipped...
        // (we expect the file to be around ~3MB, so let's keep it in memory)
        let mut decoder = GzDecoder::new(Cursor::new(&raw_bytes));
        let mut buffer_ungzipped = Vec::with_capacity(4_000_000);
        if let Err(e) = decoder.read_to_end(&mut buffer_ungzipped) {
            return Err(format!("Could not decode .vsix package as gzip: {:?}", e));
        }

        // ...but it actually is a ZIP archive underneath.
        ZipArchive::new(Cursor::new(buffer_ungzipped))
            .map_err(|e| format!("Could not read .vsix package as zip: {:?}", e))
    }

    fn query_extensions(&self, query: &ExtensionQuery) -> Result<ExtensionQueryResult, String> {
        let request = zed::http_client::HttpRequest::builder()
            .method(Post)
            .url(format!(
                "{}/_apis/public/gallery/extensionquery",
                self.base_url
            ))
            .body(serde_json::to_string(query).unwrap())
            .header(
                "accept",
                format!("application/json; api-version={}", self.api_version),
            )
            .header("content-type", "application/json")
            .header("user-agent", "Zed extension for Haxe v0.1")
            .redirect_policy(NoFollow)
            .build()
            .unwrap();

        let response = zed::http_client::fetch(&request)?;
        serde_json::from_slice(response.body.as_slice()).map_err(|e| {
            format!(
                "Could not deserialize extension query result as JSON: {:?}",
                e
            )
        })
    }

    pub fn get_latest_version(&self, author: &str, name: &str) -> Result<String, String> {
        self.query_extensions(&ExtensionQuery {
            filters: vec![ExtensionQueryFilter {
                page_number: 1,
                page_size: 1,
                criteria: vec![ExtensionQueryFilterCriteria {
                    filter_type: 7,
                    value: format!("{}.{}", author, name),
                }],
            }],
            asset_types: vec![],
            flags: 1,
        })?
        .results
        .get(0)
        .and_then(|result| result.extensions.get(0))
        .and_then(|extensions| extensions.versions.get(0))
        .map(|versions| versions.version.clone())
        .ok_or_else(|| "Extension query succeeded, but did not return any version info".into())
    }
}

#[derive(Serialize)]
struct ExtensionQuery {
    #[serde(rename = "filters")]
    pub filters: Vec<ExtensionQueryFilter>,
    #[serde(rename = "assetTypes")]
    pub asset_types: Vec<u32>,
    #[serde(rename = "flags")]
    pub flags: u32,
}

#[derive(Serialize)]
struct ExtensionQueryFilter {
    #[serde(rename = "pageNumber")]
    pub page_number: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    #[serde(rename = "criteria")]
    pub criteria: Vec<ExtensionQueryFilterCriteria>,
}

#[derive(Serialize)]
struct ExtensionQueryFilterCriteria {
    #[serde(rename = "filterType")]
    pub filter_type: u32,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Deserialize)]
struct ExtensionQueryResult {
    #[serde(rename = "results")]
    pub results: Vec<ExtensionQueryResultItem>,
}

#[derive(Deserialize)]
struct ExtensionQueryResultItem {
    #[serde(rename = "extensions")]
    pub extensions: Vec<ExtensionQueryResultItemExtension>,
}

#[derive(Deserialize)]
struct ExtensionQueryResultItemExtension {
    #[serde(rename = "versions")]
    pub versions: Vec<ExtensionQueryResultItemExtensionVersion>,
}

#[derive(Deserialize)]
struct ExtensionQueryResultItemExtensionVersion {
    #[serde(rename = "version")]
    pub version: String,
}
