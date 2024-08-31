use zed_extension_api as zed;

struct HaxeExtension {
    // ... state
}

impl zed::Extension for HaxeExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        HaxeExtension {}
    }
}

zed::register_extension!(HaxeExtension);
