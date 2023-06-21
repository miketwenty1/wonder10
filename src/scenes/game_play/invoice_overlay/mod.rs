use crate::{despawn_screen, DisplayInvoice};
use bevy::prelude::*;

use self::{
    components::Invoice,
    systems::{interact_with_copy_button, spawn_qr_code},
};
mod components;
mod systems;

pub struct InvoiceOverlay;
impl Plugin for InvoiceOverlay {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_systems(OnEnter(DisplayInvoice::On), spawn_qr_code)
            .add_systems(
                Update,
                interact_with_copy_button.run_if(in_state(DisplayInvoice::On)),
            )
            .add_systems(OnExit(DisplayInvoice::On), despawn_screen::<Invoice>);
    }
}
