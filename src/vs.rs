use std::io::{Cursor, Read};

use flate2::read::GzDecoder;
use zed_extension_api::{
    self as zed,
    http_client::{HttpMethod::Get, RedirectPolicy::*},
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

    /// Given package details, returns the URL to download that package.
    fn format_vspackage_url<'a, 'n, 'v>(
        &self,
        author: impl Into<&'a str>,
        name: impl Into<&'n str>,
        version: impl Into<&'v str>,
    ) -> String {
        format!(
            "{}/_apis/public/gallery/publishers/{}/vsextensions/{}/{}/vspackage",
            self.base_url,
            author.into(),
            name.into(),
            version.into()
        )
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
            .url(self.format_vspackage_url(author, name, version))
            .redirect_policy(NoFollow)
            .header(
                "accept",
                format!("application/vsix; api-version={}", self.api_version),
            )
            .header("accept-encoding", "gzip, deflate")
            .header("user-agent", "Zed extension for Haxe v0.1")
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
}
