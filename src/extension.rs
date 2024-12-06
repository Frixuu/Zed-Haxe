use std::{env, fs::OpenOptions, io::Write, ops::IndexMut, path::Path};

use zed_extension_api::{
    self as zed,
    serde_json::{json, Map, Value},
    settings::LspSettings,
    Command, LanguageServerId,
    Os::*,
    Result, Worktree,
};

use crate::language_server::{self};

pub struct HaxeExtension {
    language_server_binary_path: Option<String>,
    working_dir: Box<Path>,
}

impl HaxeExtension {
    /// Returns the path to the extension's working directory.
    pub fn working_dir(&self) -> &Path {
        self.working_dir.as_ref()
    }
}

impl zed::Extension for HaxeExtension {
    /// Builds a new instance of the extension.
    fn new() -> Self
    where
        Self: Sized,
    {
        let pwd = env::current_dir().unwrap();
        let working_dir = pwd.into_boxed_path();

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
            language_server_binary_path: None,
            working_dir,
        }
    }

    fn language_server_command(
        &mut self,
        id: &LanguageServerId,
        _wt: &Worktree,
    ) -> Result<Command> {
        let language_server_binary_path = match &self.language_server_binary_path {
            Some(path) => path.clone(),
            None => {
                let version = language_server::download_from_gh_if_missing(self, Some(id))?;
                let path = language_server::path_of_server_binary(self, version.as_str())
                    .to_string_lossy()
                    .to_string();
                let path = trim_leading_slash_on_windows(path);
                self.language_server_binary_path = Some(path.clone());
                path
            }
        };

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![language_server_binary_path],
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let lsp_settings = LspSettings::for_worktree("haxe-language-server", worktree).ok();

        let mut init_options = lsp_settings
            .map(|s| s.initialization_options)
            .flatten()
            .unwrap_or_else(|| Value::Object(Map::new()));

        let display_args = init_options.index_mut("displayArguments");
        match display_args {
            Value::Null => {
                // The language server requires *some* .hxml config file to be present.

                // This would be the best moment to discover .hxml files in the project root!
                // However, Zed runs its extensions in the sandbox,
                // preventing them from listing files in the worktree.
                // Maybe in the future?

                // For now, use our (almost) blank, default config
                // we created while our extension was loading:
                *display_args = json!([trim_leading_slash_on_windows(
                    self.working_dir()
                        .join("default-config.hxml")
                        .to_string_lossy()
                        .to_string()
                )]);
            }
            _ => {}
        }

        Ok(Some(init_options))
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

// See https://github.com/zed-industries/zed/issues/20559
fn trim_leading_slash_on_windows(mut s: String) -> String {
    if zed::current_platform().0 == Windows && s.starts_with(r"/") {
        s.remove(0);
    }
    s
}
