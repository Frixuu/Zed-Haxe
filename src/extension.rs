use std::{
    env,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

use zed::{Command, LanguageServerId, Worktree};
use zed_extension_api::{
    self as zed,
    serde_json::{Map, Value},
    settings::LspSettings,
    Result,
};

use crate::{
    language_server::{self},
    vs,
};

pub struct HaxeExtension {
    language_server_dir: Option<PathBuf>,
    working_dir: Box<Path>,
    vs_api_client: vs::PublicGalleryClient,
}

impl HaxeExtension {
    /// Returns the path to the extension's working directory.
    pub fn working_dir(&self) -> &Path {
        self.working_dir.as_ref()
    }
    pub fn vs_api_client(&self) -> &vs::PublicGalleryClient {
        &self.vs_api_client
    }
}

impl zed::Extension for HaxeExtension {
    /// Builds a new instance of the extension.
    fn new() -> Self
    where
        Self: Sized,
    {
        let working_dir = PathBuf::from(env::var("PWD").unwrap()).into_boxed_path();

        let default_hxml_path = working_dir.join("default-config.hxml");
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(default_hxml_path)
        {
            file.write_all("--class-path .\n--no-output\n".as_bytes())
                .ok();
        };

        HaxeExtension {
            language_server_dir: None,
            working_dir,
            vs_api_client: vs::PublicGalleryClient::new(),
        }
    }

    fn language_server_command(
        &mut self,
        id: &LanguageServerId,
        _wt: &Worktree,
    ) -> Result<Command> {
        self.language_server_dir = Some(language_server::download_if_missing(self, Some(id))?);

        let server_bin_path = self
            .language_server_dir
            .as_ref()
            .unwrap()
            .join("bin")
            .join("server.js")
            .to_string_lossy()
            .to_string();

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![server_bin_path],
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let lsp_settings = LspSettings::for_worktree("haxe-language-server", worktree).ok();

        let mut init_settings = lsp_settings
            .map(|s| s.initialization_options)
            .flatten()
            .unwrap_or_else(|| Value::Object(Map::new()));

        if init_settings.get("displayArguments").is_none() {
            let default_hxml_path = self.working_dir().join("default-config.hxml");
            init_settings["displayArguments"] = Value::Array(vec![Value::String(
                default_hxml_path.to_string_lossy().to_string(),
            )]);
        }

        Ok(Some(init_settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let lsp_settings = LspSettings::for_worktree("haxe-language-server", worktree).ok();
        let settings = lsp_settings
            .map(|s| s.settings)
            .flatten()
            .unwrap_or_else(|| Value::Object(Map::new()));

        Ok(Some(settings))
    }
}
