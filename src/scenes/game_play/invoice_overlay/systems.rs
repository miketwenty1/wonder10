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

#[cfg(web_sys_unstable_apis)]
pub fn spawn_qr_code(mut commands: Commands, invoice_data: Res<InvoiceDataFromServer>) {
    use crate::scenes::game_play::invoice_overlay::QrVersion;

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

    let image = match egui_extras::RetainedImage::from_svg_bytes_with_size(
        "invoicesvg",
        image.as_bytes(),
        egui_extras::image::FitTo::Original,
    ) {
        Ok(img) => img,
        Err(err) => {
            println!("Failed to create RetainedImage: {:?}", err);
            return; // or handle the error in another way, e.g., provide a default image or show an error message to the user
        }
    };

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
