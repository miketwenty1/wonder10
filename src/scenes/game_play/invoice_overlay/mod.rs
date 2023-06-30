use crate::{despawn_screen, DisplayInvoiceQr};
use bevy::prelude::*;
use systems::spawn_qr_code;

use self::{components::InvoiceOverlay, systems::update_qr_code};
pub mod components;
pub mod systems;

// #[derive(Resource, Deref)]
// pub struct MyQr(pub RetainedImage);

impl Plugin for InvoiceOverlay {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(DisplayInvoiceQr::On), spawn_qr_code)
            .add_systems(
                Update,
                update_qr_code.run_if(in_state(DisplayInvoiceQr::On)),
            )
            .add_systems(
                OnExit(DisplayInvoiceQr::On),
                despawn_screen::<InvoiceOverlay>,
            );
    }
}
