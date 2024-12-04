use zed_extension_api::{self as zed};

mod extension;
mod language_server;
mod vs;

zed::register_extension!(crate::extension::HaxeExtension);
