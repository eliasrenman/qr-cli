use crate::daemon::Daemon;
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
use arboard::{Clipboard, ImageData};
use image::Rgba;
use qrcode::QrCode;
use std::borrow::Cow;
pub struct Generate {}

impl Generate {
    // pub fn svg(url: String) {
    //     let code = QrCode::with_version(url, Version::Micro(2), EcLevel::L).unwrap();
    //     let image = code
    //         .render()
    //         .min_dimensions(200, 200)
    //         .dark_color(svg::Color("#000000"))
    //         .light_color(svg::Color("#ffffff"))
    //         .build();
    // }
    pub fn png(url: String) {
        let code = QrCode::new(url).unwrap();
        // Render the bits into an image.
        let image = code
            .render::<Rgba<u8>>()
            .min_dimensions(200, 200)
            .dark_color(Rgba([0, 0, 0, 255]))
            .light_color(Rgba([255, 255, 255, 255]))
            .build();
        let image_data = ImageData {
            width: usize::try_from(image.width()).unwrap(),
            height: usize::try_from(image.height()).unwrap(),
            bytes: Cow::from(image.into_vec()),
        };
        Generate::set_image(image_data);
    }

    fn set_image(image_data: ImageData<'_>) {
        #[cfg(target_os = "linux")]
        if Daemon::in_daemon() {
            // print!("LINUX: Successfully copied png to clipboard\n");
            let _ = Clipboard::new().unwrap().set().wait().image(image_data);
            return;
        }

        let mut clipboard = Clipboard::new().unwrap();
        let result = clipboard.set_image(image_data);
        if result.is_err() {
            println!("Failed copying png to clipboard");

            println!("err: {:?}", result.unwrap_err().to_string());
        } else {
            println!("Successfully copied png to clipboard");
        }
    }
}
