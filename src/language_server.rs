use std::{fs, path::PathBuf};

use crate::extension::HaxeExtension;
use zed::DownloadedFileType::Uncompressed;
use zed_extension_api::{
    self as zed, GithubReleaseOptions, LanguageServerId,
    LanguageServerInstallationStatus::{self, *},
};

/// If an ID of a language server is provided, sets this server's installation status.
fn set_maybe_status(id: Option<&LanguageServerId>, status: &LanguageServerInstallationStatus) {
    if let Some(id) = id {
        zed::set_language_server_installation_status(id, status);
    }
}

pub fn path_of_server_dir(extension: &HaxeExtension, version: &str) -> PathBuf {
    extension
        .working_dir()
        .join("language-server")
        .join(version)
}

pub fn path_of_server_binary(extension: &HaxeExtension, version: &str) -> PathBuf {
    path_of_server_dir(extension, version).join("server.js")
}

pub fn is_version_installed(extension: &HaxeExtension, version: &str) -> bool {
    let path = path_of_server_binary(extension, version);
    fs::metadata(&path).map_or(false, |s| s.is_file())
}

pub fn download_from_gh_if_missing(
    extension: &HaxeExtension,
    id: Option<&LanguageServerId>,
) -> Result<String, String> {
    set_maybe_status(id, &CheckingForUpdate);

    let latest_release = zed::latest_github_release(
        // TODO replace with upstream when they start publishing releases
        "frixuu/haxe-language-server",
        GithubReleaseOptions {
            require_assets: true,
            pre_release: false,
        },
    )?;

    set_maybe_status(id, &None);

    let version = latest_release.version;
    let asset = latest_release
        .assets
        .iter()
        .find(|a| a.name == "server.js")
        .unwrap();

    if !is_version_installed(extension, version.as_str()) {
        download_from_gh(extension, id, asset.download_url.as_str(), version.as_str())?;
    }

    Ok(version)
}

/// Forcefully downloads a specified version of the language server.
fn download_from_gh(
    extension: &HaxeExtension,
    id: Option<&LanguageServerId>,
    url: &str,
    version: &str,
) -> Result<(), String> {
    fs::create_dir_all(path_of_server_dir(extension, version))
        .map_err(|e| format!("Could not create language server directory: {e:?}"))?;

    set_maybe_status(id, &Downloading);
    match zed::download_file(
        url,
        path_of_server_binary(extension, version).to_str().unwrap(),
        Uncompressed,
    ) {
        Ok(_) => {
            set_maybe_status(id, &None);
            Ok(())
        }
        Err(e) => {
            set_maybe_status(id, &Failed(e.clone()));
            Err(format!("Could not download language server: {e:?}"))
        }
    }
}
