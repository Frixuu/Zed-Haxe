use zed_extension_api::{self as zed};

mod extension;
mod language_server;

zed::register_extension!(crate::extension::HaxeExtension);
