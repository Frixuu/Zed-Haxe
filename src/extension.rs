use std::{env, fs::OpenOptions, io::Write, ops::IndexMut, path::Path};

use zed_extension_api::{
    self as zed,
    lsp::{Completion, CompletionKind, Symbol, SymbolKind},
    serde_json::{json, Map, Value},
    settings::LspSettings,
    CodeLabel, CodeLabelSpan, Command, LanguageServerId,
    Os::*,
    Result, Worktree,
};

use crate::language_server::{self};

pub struct HaxeExtension {
    language_server_binary_path: Option<String>,
    language_server_version: Option<String>,
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

        let last_version_path = working_dir.join("version.txt");
        let language_server_version = std::fs::read_to_string(last_version_path).ok();

        HaxeExtension {
            language_server_binary_path: None,
            language_server_version,
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
                // Try to fetch the latest version of the language server and download it
                let version = match language_server::download_from_gh_if_missing(self, Some(id)) {
                    Ok(version) => {
                        let last_version_path = self.working_dir.join("version.txt");
                        std::fs::write(last_version_path, &version).ok();
                        self.language_server_version = Some(version.clone());
                        version
                    }
                    Err(_) => match &self.language_server_version {
                        // If downloading fails, try to use the last known version (if one was set)
                        Some(version) => {
                            if language_server::is_version_installed(&self, version.as_str()) {
                                version.clone()
                            } else {
                                return Err(format!(
                                    "Language server version \"{version}\" was set as fallback, but it does not exist on disk"
                                ));
                            }
                        }
                        // Or fail, if no version was set (extension was never run)
                        None => return Err(
                            "Failed to download the language server. Also, no known version installed previously"
                                .into()),
                    },
                };
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

    fn label_for_completion(
        &self,
        _: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        let name = completion.label.trim();
        let (prefix, suffix) = match completion.kind? {
            CompletionKind::Function => ("function NOT_REAL_", "(){}"),
            CompletionKind::Method => ("class NOT_REAL { public function NOT_REAL_", "(){} }"),
            CompletionKind::Class => ("class NOT_REAL_", "{}"),
            CompletionKind::Interface => ("interface NOT_REAL_", "{}"),
            CompletionKind::Variable => ("var NOT_REAL_", ":Any;"),
            CompletionKind::Constant => ("public static inline final NOT_REAL_", ":Any = null;"),
            CompletionKind::Field => ("class NOT_REAL { var NOT_REAL_", ":Any; }"),
            CompletionKind::Property => ("class NOT_REAL { var NOT_REAL_", "(get, set):Any; }"),
            CompletionKind::Enum => ("enum NOT_REAL_", "{}"),
            CompletionKind::EnumMember => ("enum NOT_REAL { NOT_REAL_", "; }"),
            CompletionKind::Constructor => ("enum NOT_REAL { NOT_REAL_", "(foo:Any); }"),
            CompletionKind::TypeParameter => ("class NOT_REAL<", ">{}"),
            CompletionKind::Keyword => ("", ""),
            _ => return None,
        };

        let faux_code = format!("{prefix}{name}{suffix}");
        Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(
                prefix.len()..(prefix.len() + name.len()),
            )],
            filter_range: (0..name.len()).into(),
            code: faux_code,
        })
    }

    fn label_for_symbol(&self, _: &LanguageServerId, symbol: Symbol) -> Option<CodeLabel> {
        let name = symbol.name.trim();
        let (prefix, suffix) = match symbol.kind {
            SymbolKind::Function => ("function NOT_REAL_", "(){}"),
            SymbolKind::Method => ("class NOT_REAL { public function NOT_REAL_", "(){} }"),
            SymbolKind::Class => ("class NOT_REAL_", "{}"),
            SymbolKind::Interface => ("interface NOT_REAL_", "{}"),
            SymbolKind::Variable => ("var NOT_REAL_", ":Any;"),
            SymbolKind::Constant => ("public static inline final NOT_REAL_", ":Any = null;"),
            SymbolKind::Field => ("class NOT_REAL { var NOT_REAL_", ":Any; }"),
            SymbolKind::Property => ("class NOT_REAL { var NOT_REAL_", "(get, set):Any; }"),
            SymbolKind::Enum => ("enum NOT_REAL_", "{}"),
            SymbolKind::EnumMember => ("enum NOT_REAL { NOT_REAL_", "; }"),
            SymbolKind::Constructor => ("enum NOT_REAL { NOT_REAL_", "(foo:Any); }"),
            SymbolKind::TypeParameter => ("class NOT_REAL<", ">{}"),
            _ => return None,
        };

        let faux_code = format!("{prefix}{name}{suffix}");
        Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(
                prefix.len()..(prefix.len() + name.len()),
            )],
            filter_range: (0..name.len()).into(),
            code: faux_code,
        })
    }
}

// See https://github.com/zed-industries/zed/issues/20559
fn trim_leading_slash_on_windows(mut s: String) -> String {
    if zed::current_platform().0 == Windows && s.starts_with(r"/") {
        s.remove(0);
    }
    s
}
