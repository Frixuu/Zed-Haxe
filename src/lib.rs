use zed_extension_api::{self as zed};

mod extension;
mod helper_scripts;
mod language_server;

zed::register_extension!(crate::extension::HaxeExtension);
