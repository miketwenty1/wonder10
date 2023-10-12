use crate::comms::resources::InvoiceDataFromServer;
use crate::scenes::game_play::blocks_grid::SelectedBlock;
use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};
use egui_extras::RetainedImage;
use qrcode::{render::svg, Version};
use qrcode::{EcLevel, QrCode};
#[derive(Resource, Deref)]
pub struct MyQr(pub RetainedImage);

enum QrVersion {
    Normal(u8),
}

impl QrVersion {
    fn for_length(length: usize) -> Option<Self> {
        // Alphanumeric capacities for versions 1-40 with L (Low) error correction level.
        let capacities = [
            (25, 1),
            (47, 2),
            (77, 3),
            (114, 4),
            (154, 5),
            (195, 6),
            (224, 7),
            (279, 8),
            (335, 9),
            (395, 10),
            (468, 11),
            (535, 12),
            (619, 13),
            (667, 14),
            (758, 15),
            (854, 16),
            (938, 17),
            (1046, 18),
            (1153, 19),
            (1249, 20),
            (1352, 21),
            (1460, 22),
            (1588, 23),
            (1704, 24),
            (1853, 25),
            (1990, 26),
            (2132, 27),
            (2223, 28),
            (2369, 29),
            (2520, 30),
            (2677, 31),
            (2840, 32),
            (3009, 33),
            (3183, 34),
            (3351, 35),
            (3537, 36),
            (3720, 37),
            (3927, 38),
            (4087, 39),
            (4296, 40),
        ];

        for &(cap, version) in &capacities {
            if length <= cap {
                return Some(QrVersion::Normal(version));
            }
        }

        None
    }
}

#[cfg(web_sys_unstable_apis)]
pub fn spawn_qr_code(mut commands: Commands, invoice_data: Res<InvoiceDataFromServer>) {
    info!("spawning QR code");

    let version_number = match QrVersion::for_length(invoice_data.invoice.len()) {
        Some(QrVersion::Normal(version)) => version,
        None => {
            println!("Data is too long for the supported QR Code versions.");
            return; // or handle the error in another way, e.g., set to a default version
        }
    };

    info!("version is {}", version_number);

    let precode = QrCode::with_version(
        invoice_data.invoice.to_string().to_ascii_uppercase(),
        Version::Normal(version_number.into()),
        EcLevel::L,
    );

    let code = match precode {
        Ok(qr_code) => qr_code,
        Err(err) => {
            println!("Failed to generate QR Code: {:?}", err);
            return; // Or handle the error in another way if you prefer
        }
    };

    let image = code
        .render::<svg::Color>()
        .min_dimensions(200, 200)
        .max_dimensions(300, 300)
        .dark_color(svg::Color("#800000"))
        .light_color(svg::Color("#ffff80"))
        .build();

    let image = egui_extras::RetainedImage::from_svg_bytes_with_size(
        "invoicesvg",
        image.as_bytes(),
        egui_extras::image::FitTo::Original,
    )
    .unwrap();

    commands.insert_resource(MyQr(image));
}

#[cfg(web_sys_unstable_apis)]
pub fn update_qr_code(
    qr: Res<MyQr>,
    invoice_data: Res<InvoiceDataFromServer>,
    mut egui_context: EguiContexts,
    selected_location: Res<SelectedBlock>,
    mut local_copy: Local<bool>,
) {
    egui::Window::new(format!(
        "BTC Invoice for Height {}",
        selected_location.height
    ))
    .show(egui_context.ctx_mut(), |ui| {
        // Size to smallest square to preserve dimensions
        let bevy_egui::egui::Vec2 { x, y } = ui.available_size();
        let smaller = x.min(y);
        qr.0.show_size(ui, bevy_egui::egui::Vec2::new(smaller, smaller));

        let button_min_size = bevy_egui::egui::Vec2 { x: 120.0, y: 60.0 };
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            if ui
                .add(egui::Button::new("Copy to Clipboard").min_size(button_min_size))
                .clicked()
            //&& !(*local_copy)
            //.clicked() should be used but for some reason isnt working on mobile.
            {
                *local_copy = true;
                let qrcode_str = invoice_data.invoice.clone();
                use wasm_bindgen_futures::spawn_local;
                spawn_local(async move {
                    let window = web_sys::window().expect("window"); // { obj: val };
                    let nav = window.navigator().clipboard();
                    match nav {
                        Some(a) => {
                            let p = a.write_text(&qrcode_str);
                            let _result = wasm_bindgen_futures::JsFuture::from(p)
                                .await
                                .expect("clipboard populated");
                            info!("clippyboy worked");
                        }
                        None => {
                            warn!("failed to copy clippyboy");
                        }
                    };
                });
            };
        });
    });
}
