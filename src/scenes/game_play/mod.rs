mod block_details_overlay;
pub mod blocks_grid;
pub mod events;
pub mod game_layout;
mod invoice_overlay;
mod movement;
mod ulam;
mod update_systems;

use bevy::prelude::*;

use crate::GameState;

use self::block_details_overlay::BlockDetailsMenuPlugin;
use self::blocks_grid::SelectedBlock;
use self::events::{
    BlockButtonSelected, BlockDetailClick, BuyBlockRequest, PlayerMove, ServerBlockchainBlockIn,
    ServerGameBocksIn,
};
use self::game_layout::spawn_layout;
use self::invoice_overlay::InvoiceOverlay;
use self::movement::update_blocks_from_server_on_move;
use self::update_systems::{
    button_block_details, button_interaction_system, update_listen_for_player_move,
    update_listen_for_player_select,
};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const SELECTED_BUTTON: Color = Color::rgb(0.15, 0.15, 1.0);
const HOVERED_BUTTON: Color = Color::rgb(0.75, 0.55, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .init_resource::<SelectedBlock>()
            .add_event::<BlockButtonSelected>()
            .add_event::<PlayerMove>()
            .add_event::<ServerGameBocksIn>()
            .add_event::<ServerBlockchainBlockIn>()
            .add_event::<BlockDetailClick>()
            .add_event::<BuyBlockRequest>()
            .add_systems(OnEnter(GameState::Game), spawn_layout)
            // .add_systems(
            //     Startup,
            //     init_blocks::init_block_data.run_if(in_state(GameState::Game)),
            // )
            .add_systems(
                Update,
                update_listen_for_player_move.run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                update_listen_for_player_select.run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                button_interaction_system.run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                button_block_details.run_if(in_state(GameState::Game)),
            )
            .add_plugin(BlockDetailsMenuPlugin)
            .add_plugin(InvoiceOverlay)
            .add_systems(
                Update,
                update_blocks_from_server_on_move.run_if(in_state(GameState::Game)),
            );
    }
}
