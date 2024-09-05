use std::{env, path::PathBuf};

use zed::{Command, LanguageServerId, Worktree};
use zed_extension_api::{
    self as zed,
    serde_json::{Map, Value},
    settings::LspSettings,
    Result,
};

use crate::language_server::{self};

pub struct HaxeExtension {
    language_server_dir: Option<PathBuf>,
}

impl zed::Extension for HaxeExtension {
    /// Builds a new instance of the extension.
    fn new() -> Self
    where
        Self: Sized,
    {
        HaxeExtension {
            language_server_dir: None,
        }
    }

    fn language_server_command(
        &mut self,
        id: &LanguageServerId,
        _wt: &Worktree,
    ) -> Result<Command> {
        self.language_server_dir = Some(language_server::download_if_missing(Some(id))?);

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
            .as_ref()
            .map(|s| s.initialization_options.clone())
            .flatten()
            .unwrap_or_else(|| Value::Object(Map::new()));

        if init_settings.get("displayArguments").is_none() {
            init_settings["displayArguments"] =
                Value::Array(vec![Value::String("build.hxml".to_owned())]);
        }

        Ok(Some(init_settings))
    }
}

pub fn working_dir() -> PathBuf {
    PathBuf::from(env::var("PWD").unwrap())
}
