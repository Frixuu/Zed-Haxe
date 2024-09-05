use std::path::PathBuf;

use const_format::formatcp;
use zed::{DownloadedFileType, LanguageServerInstallationStatus::*};
use zed_extension_api::{self as zed, LanguageServerId, LanguageServerInstallationStatus};

pub const MARKETPLACE_API_URL: &str = "https://marketplace.visualstudio.com/_apis/public";
pub const VSHAXE_AUTHOR: &str = "nadako";
pub const VSHAXE_NAME: &str = "vshaxe";
pub const VSHAXE_VERSION: &str = "2.32.1";
pub const VSHAXE_DOWNLOAD_URL: &str = formatcp!(
    "{url}/gallery/publishers/{author}/vsextensions/{name}/{version}/vspackage",
    url = MARKETPLACE_API_URL,
    author = VSHAXE_AUTHOR,
    name = VSHAXE_NAME,
    version = VSHAXE_VERSION
);

/// If an ID of a language server is provided, sets this server's installation status.
fn set_maybe_status(id: Option<&LanguageServerId>, status: &LanguageServerInstallationStatus) {
    if let Some(id) = id {
        zed::set_language_server_installation_status(id, status);
    }
}

/// Forcefully downloads the latest known version of the language server.
pub fn download_fresh(id: Option<&LanguageServerId>) -> Result<PathBuf, String> {
    set_maybe_status(id, &Downloading);
    zed::download_file(
        VSHAXE_DOWNLOAD_URL,
        formatcp!("vshaxe-{VSHAXE_VERSION}"),
        DownloadedFileType::Zip, //.vsix are zips
    )?;
    // WASM sandbox does not permit us to query the filesystem, assume download succeeded
    set_maybe_status(id, &None);
    Ok(instance_dir_path())
}

pub fn instance_dir_path() -> PathBuf {
    let mut path = crate::extension::working_dir();
    path.push(formatcp!("vshaxe-{VSHAXE_VERSION}"));
    path.push("extension");
    path
}
