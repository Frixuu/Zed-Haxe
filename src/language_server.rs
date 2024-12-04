use std::path::PathBuf;

use crate::extension::HaxeExtension;
use zed_extension_api::{
    self as zed, LanguageServerId,
    LanguageServerInstallationStatus::{self, *},
};

pub const VSHAXE_AUTHOR: &str = "nadako";
pub const VSHAXE_NAME: &str = "vshaxe";

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
    let vs_api_client = extension.vs_api_client();
    let mut vs_extension_archive =
        vs_api_client.download_package_as_zip(VSHAXE_AUTHOR, VSHAXE_NAME, version)?;

    // Extract the package in our extension's working directory
    let path = extension.working_dir().join(format!("vshaxe-{version}"));
    match vs_extension_archive.extract(path) {
        Ok(_) => {
            set_maybe_status(id, &None);
            Ok(instance_dir_path(extension, version))
        }
        Err(e) => {
            set_maybe_status(
                id,
                &Failed(format!(
                    "Could not extract language server package to disk: {e:?}"
                )),
            );
            Err(e.to_string())
        }
    }
}

pub fn is_version_installed(extension: &HaxeExtension, version: &str) -> bool {
    let path = instance_dir_path(extension, version);
    std::fs::metadata(&path).map_or(false, |s| s.is_dir())
}

pub fn download_if_missing(
    extension: &HaxeExtension,
    id: Option<&LanguageServerId>,
) -> Result<PathBuf, String> {
    set_maybe_status(id, &CheckingForUpdate);
    let vs_api_client = extension.vs_api_client();
    let version = vs_api_client
        .get_latest_version(VSHAXE_AUTHOR, VSHAXE_NAME)
        .unwrap_or("2.32.2".into());
    set_maybe_status(id, &None);

    if is_version_installed(extension, version.as_str()) {
        Ok(instance_dir_path(extension, version.as_str()))
    } else {
        download_fresh(extension, id, version.as_str())
    }
}

pub fn instance_dir_path(extension: &HaxeExtension, version: &str) -> PathBuf {
    extension
        .working_dir()
        .join(format!("vshaxe-{version}"))
        .join("extension")
}
