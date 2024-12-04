use std::{
    io::{Cursor, Read},
    path::PathBuf,
};

use flate2::read::GzDecoder;
use zed_extension_api::{
    self as zed,
    http_client::{fetch, HttpMethod::Get, HttpRequest, RedirectPolicy::*},
    LanguageServerId,
    LanguageServerInstallationStatus::{self, *},
};
use zip::ZipArchive;

use crate::extension;

pub const MARKETPLACE_API_URL: &str = "https://marketplace.visualstudio.com/_apis/public";
pub const VSHAXE_AUTHOR: &str = "nadako";
pub const VSHAXE_NAME: &str = "vshaxe";
pub const VSHAXE_VERSION: &str = "2.32.2";

fn download_url(version: &str) -> String {
    format!(
        "{}/gallery/publishers/{}/vsextensions/{}/{}/vspackage",
        MARKETPLACE_API_URL, VSHAXE_AUTHOR, VSHAXE_NAME, version
    )
}

/// If an ID of a language server is provided, sets this server's installation status.
fn set_maybe_status(id: Option<&LanguageServerId>, status: &LanguageServerInstallationStatus) {
    if let Some(id) = id {
        zed::set_language_server_installation_status(id, status);
    }
}

/// Forcefully downloads a specified version of the language server.
pub fn download_fresh(id: Option<&LanguageServerId>, version: &str) -> Result<PathBuf, String> {
    set_maybe_status(id, &Downloading);

    // Download the Visual Studio extension package.
    // This distribution method was chosen, because the alternatives are less convenient:
    // - The GitHub releases for the language server contain only the source code,
    //   which would need to be compiled (and additional packages installed) to run.
    // - The GitHub packages are precompiled, but the repo does not allow anonymous downloads.
    let vsix_bytes = fetch(
        &HttpRequest::builder()
            .method(Get)
            .url(download_url(version))
            .redirect_policy(FollowLimit(5))
            .header("accept", "application/vsix")
            .header("accept-encoding", "gzip, deflate")
            .header("user-agent", "Zed extension for Haxe v0.1")
            .build()?,
    )?
    .body;

    // The .vsix release is gzipped...
    let mut decoder = GzDecoder::new(Cursor::new(&vsix_bytes));
    let mut buffer_intermediate = Vec::with_capacity(4_000_000);
    decoder
        .read_to_end(&mut buffer_intermediate)
        .map_err(|e| e.to_string())?;

    // but it actually is a ZIP archive underneath.
    // Extract it somewhere in our extension's working directory
    ZipArchive::new(Cursor::new(&buffer_intermediate))
        .map_err(|e| e.to_string())?
        .extract(extension::working_dir().join(format!("vshaxe-{version}")))
        .map_err(|e| e.to_string())?;

    set_maybe_status(id, &None);
    Ok(instance_dir_path(version))
}

pub fn is_version_installed(version: &str) -> bool {
    let path = instance_dir_path(version);
    std::fs::metadata(&path).map_or(false, |s| s.is_dir())
}

pub fn download_if_missing(id: Option<&LanguageServerId>) -> Result<PathBuf, String> {
    if is_version_installed(VSHAXE_VERSION) {
        Ok(instance_dir_path(VSHAXE_VERSION))
    } else {
        download_fresh(id, VSHAXE_VERSION)
    }
}

pub fn instance_dir_path(version: &str) -> PathBuf {
    let mut path = crate::extension::working_dir();
    path.push(format!("vshaxe-{version}"));
    path.push("extension");
    path
}
