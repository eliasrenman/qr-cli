use clap::Parser;

/// This is used to generate a QR Code png image which will be copied to the clipboard.
#[derive(Parser)]
pub struct QrUrl {
    /// Url endpoint of the QR Code to be generated.
    pub url: String,
    // /// The output format of the QR Code.
    // pub format: Format,
    #[arg(hide(true), default_value(""))]
    pub __internal_deamonize: String,
}

#[derive(clap::ValueEnum, Clone, strum_macros::Display)]
pub enum Format {
    Svg,
    Png,
    Webp,
}
