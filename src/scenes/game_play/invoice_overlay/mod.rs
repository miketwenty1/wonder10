use crate::{despawn_screen, DisplayInvoiceQr};
use bevy::prelude::*;

use self::{components::InvoiceOverlay, systems::spawn_qr_code};
pub mod components;
pub mod systems;

// #[derive(Resource, Deref)]
// pub struct MyQr(pub RetainedImage);

impl Plugin for InvoiceOverlay {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            //.insert_resource(WinitSettings::desktop_app())
            // .add_systems(OnEnter(DisplayInvoiceQr::On), spawn_qr_code)
            // .add_systems(
            //     Update,
            //     interact_with_copy_button.run_if(in_state(DisplayInvoice::On)),
            // )
            .add_systems(
                OnExit(DisplayInvoiceQr::On),
                despawn_screen::<InvoiceOverlay>,
            );
    }
}
