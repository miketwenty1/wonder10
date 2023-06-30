use crate::comms::resources::InvoiceDataFromServer;
use crate::PlayerLocation;
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
    info!("spawning QR code");

    let code = QrCode::with_version(
        invoice_data.invoice.to_string().to_ascii_uppercase(),
        Version::Normal(9),
        EcLevel::L,
    )
    .unwrap();

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
    player_location: Res<PlayerLocation>,
    mut local_copy: Local<bool>,
) {
    egui::Window::new(format!("BTC Invoice for Height {}", player_location.0)).show(
        egui_context.ctx_mut(),
        |ui| {
            // Size to smallest square to preserve dimensions
            let bevy_egui::egui::Vec2 { x, y } = ui.available_size();
            let smaller = x.min(y);
            qr.0.show_size(ui, bevy_egui::egui::Vec2::new(smaller, smaller));

            let button_min_size = bevy_egui::egui::Vec2 { x: 120.0, y: 60.0 };
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                if ui
                    .add(egui::Button::new("Copy to Clipboard").min_size(button_min_size))
                    .enabled()
                    && !(*local_copy)
                //.clicked() should be used but for some reason isnt working on mobile.
                //
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
        },
    );
}
