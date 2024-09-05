use std::{env, path::PathBuf};

use zed::{Command, LanguageServerId, Worktree};
use zed_extension_api::{self as zed, serde_json::json, Result};

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
        if self.language_server_dir.is_none() {
            self.language_server_dir = Some(language_server::download_fresh(Some(id))?);
        }

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
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(Some(json!({ "displayArguments": ["build.hxml"] })))
    }
}

pub fn working_dir() -> PathBuf {
    PathBuf::from(env::var("PWD").unwrap())
}
