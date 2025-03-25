use zed_extension_api as zed;

pub struct TydExtension {}

impl zed::Extension for TydExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        TydExtension {}
    }
}

zed::register_extension!(TydExtension);
