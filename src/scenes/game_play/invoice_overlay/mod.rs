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
