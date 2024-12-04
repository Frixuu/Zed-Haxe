use std::path::PathBuf;

use crate::{extension::HaxeExtension, vs};
use zed_extension_api::{
    self as zed, LanguageServerId,
    LanguageServerInstallationStatus::{self, *},
};

pub const VSHAXE_VERSION: &str = "2.32.2";

/// If an ID of a language server is provided, sets this server's installation status.
fn set_maybe_status(id: Option<&LanguageServerId>, status: &LanguageServerInstallationStatus) {
    if let Some(id) = id {
        zed::set_language_server_installation_status(id, status);
    }
}

/// Forcefully downloads a specified version of the language server.
pub fn download_fresh(
    extension: &HaxeExtension,
    id: Option<&LanguageServerId>,
    version: &str,
) -> Result<PathBuf, String> {
    set_maybe_status(id, &Downloading);

    // Download the Visual Studio extension package.
    // This distribution method was chosen, because the alternatives are less convenient:
    // - The GitHub releases for the language server contain only the source code,
    //   which would need to be compiled (and additional packages installed) to run.
    // - The GitHub packages are precompiled, but the repo does not allow anonymous downloads.
    let client = vs::PublicGalleryClient::new();
    let mut vs_extension_archive = client.download_package_as_zip("nadako", "vshaxe", version)?;

    // Extract the package in our extension's working directory
    vs_extension_archive
        .extract(extension.working_dir().join(format!("vshaxe-{version}")))
        .map_err(|e| e.to_string())?;

    set_maybe_status(id, &None);
    Ok(instance_dir_path(extension, version))
}

pub fn is_version_installed(extension: &HaxeExtension, version: &str) -> bool {
    let path = instance_dir_path(extension, version);
    std::fs::metadata(&path).map_or(false, |s| s.is_dir())
}

pub fn download_if_missing(
    extension: &HaxeExtension,
    id: Option<&LanguageServerId>,
) -> Result<PathBuf, String> {
    if is_version_installed(extension, VSHAXE_VERSION) {
        Ok(instance_dir_path(extension, VSHAXE_VERSION))
    } else {
        download_fresh(extension, id, VSHAXE_VERSION)
    }
}

pub fn instance_dir_path(extension: &HaxeExtension, version: &str) -> PathBuf {
    extension
        .working_dir()
        .join(format!("vshaxe-{version}"))
        .join("extension")
}
