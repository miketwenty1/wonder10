use bevy::prelude::*;

use crate::comms::resources::InvoiceDataFromServer;

// pub fn interact_with_copy_button() {
//     info!("COPY BUTTON Iteraction!");
// }

// #[cfg(web_sys_unstable_apis)]
// pub fn update_qr(
//     qr: Res<MyQr>,
//     mut egui_context: ResMut<EguiContext>,
//     qrcode_str: ResMut<CurrentQrString>,
// ) {
//     egui::Window::new("BTC Invoice").show(egui_context.ctx_mut(), |ui| {
//         // Size to smallest square to preserve dimensions
//         let bevy_egui::egui::Vec2 { x, y } = ui.available_size();
//         let smaller = x.min(y);
//         qr.0.show_size(ui, bevy_egui::egui::Vec2::new(smaller, smaller));

//         let button_min_size = bevy_egui::egui::Vec2 { x: 120.0, y: 60.0 };
//         ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
//             if ui
//                 .add(egui::Button::new("Copy to Clipboard").min_size(button_min_size))
//                 .clicked()
//             {
//                 let qrcode_str = qrcode_str.clone();
//                 use wasm_bindgen_futures::spawn_local;
//                 spawn_local(async move {
//                     let window = web_sys::window().expect("window"); // { obj: val };
//                     let nav = window.navigator().clipboard();
//                     match nav {
//                         Some(a) => {
//                             let p = a.write_text(&qrcode_str);
//                             let _result = wasm_bindgen_futures::JsFuture::from(p)
//                                 .await
//                                 .expect("clipboard populated");
//                             info!("clippyboy worked");
//                         }
//                         None => {
//                             warn!("failed to copy clippyboy");
//                         }
//                     };
//                 });
//                 //info!("{}", qrcode_str.to_string());
//             };
//         });
//     });
// }

pub fn spawn_qr_code(
    mut commands: Commands,
    invoice_data: Res<InvoiceDataFromServer>,
    //mut images: ResMut<Assets<Image>>,
) {
    info!("spawning QR code");
    // let invoice_str = &invoice_data.invoice;
    // let qr_code = QrCode::with_version(invoice_str, Version::Normal(9), EcLevel::L).unwrap();

    // let image = qr_code
    //     .render::<svg::Color>()
    //     .min_dimensions(200, 200)
    //     .max_dimensions(300, 300)
    //     .dark_color(svg::Color("#800000"))
    //     .light_color(svg::Color("#ffff80"))
    //     .build();

    // let a = egui_extras::RetainedImage::from_svg_bytes_with_size(
    //     "invoicesvg",
    //     image.as_bytes(),
    //     egui_extras::image::FitTo::Original,
    // )
    // .unwrap();

    // // Cache QR code to be used later
    // commands.insert_resource(MyQr(a));

    // let my_vector = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // let image = Image {
    //     data: my_vector,
    //     ..Default::default()
    // };

    //let image_handle = images.add(image);

    // commands.spawn(ImageBundle {
    //     image: UiImage::new(image_handle),
    //     style: Style {
    //         size: Size {
    //             width: Val::Percent(100.0),
    //             height: Val::Percent(100.0),
    //         },
    //         ..Default::default()
    //     },
    //     background_color: Color::WHITE.into(),
    //     ..Default::default()
    // });
}

// fn spawn_qr_code(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let svg = asset_server.load("neutron_star.svg");
//     //commands.spawn(Camera2dBundle::default());
//     commands.spawn(Svg2dBundle {
//         svg,
//         origin: Origin::Center,
//         ..Default::default()
//     });
// }
