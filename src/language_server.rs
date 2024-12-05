use std::{
    fs::{self},
    path::PathBuf,
};

use crate::extension::HaxeExtension;
use zed_extension_api::{
    self as zed,
    http_client::{HttpMethod::*, RedirectPolicy::*},
    GithubReleaseOptions, LanguageServerId,
    LanguageServerInstallationStatus::{self, *},
};

const USER_AGENT: &'static str = "Zed extension for Haxe v0.2";

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
    let server_dir = path_of_server_dir(extension, version);
    fs::create_dir_all(&server_dir)
        .map_err(|e| format!(
            "Tried to ensure a directory for the language server exists before downloading it, but could not create it: {e:?}"
        ))?;

    set_maybe_status(id, &Downloading);

    let request = zed::http_client::HttpRequest::builder()
        .method(Get)
        .url(url)
        .header("user-agent", USER_AGENT)
        .redirect_policy(FollowLimit(5))
        .build()
        .unwrap();

    let response = zed::http_client::fetch(&request).map_err(|e| {
        format!("Tried to fetch the language server binary, but something happened: {e:?}")
    })?;

    fs::write(path_of_server_binary(extension, version), &response.body)
        .map_err(|e| format!("Could not save the language server binary to disk: {e:?}"))?;

    set_maybe_status(id, &None);
    Ok(())
}
