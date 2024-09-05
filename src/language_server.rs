use std::path::PathBuf;

use zed::{DownloadedFileType, LanguageServerInstallationStatus::*};
use zed_extension_api::{self as zed, LanguageServerId, LanguageServerInstallationStatus};

pub const MARKETPLACE_API_URL: &str = "https://marketplace.visualstudio.com/_apis/public";
pub const VSHAXE_AUTHOR: &str = "nadako";
pub const VSHAXE_NAME: &str = "vshaxe";
pub const VSHAXE_VERSION: &str = "2.32.1";

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
    zed::download_file(
        download_url(version).as_ref(),
        format!("vshaxe-{version}").as_ref(),
        DownloadedFileType::Zip, //.vsix are zips
    )?;
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
