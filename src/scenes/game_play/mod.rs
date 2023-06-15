pub mod blocks_grid;
pub mod events;
pub mod game_layout;
mod init_blocks;
mod movement;
mod overlays;
mod ulam;
mod update_systems;

use bevy::prelude::*;

use crate::GameState;

use self::blocks_grid::SelectedBlock;
use self::events::{
    BlockButtonSelected, BlockDetailClick, PlayerMove, ServerBlockchainBlocksIn, ServerGameBocksIn,
};
use self::game_layout::spawn_layout;
use self::movement::update_blocks_from_server_on_move;
use self::update_systems::{
    button_block_details, button_interaction_system, update_listen_for_player_move,
    update_listen_for_player_select,
};
use overlays::block_details_overlay::display_blockchain_block_details;

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
            .add_event::<ServerBlockchainBlocksIn>()
            .add_event::<BlockDetailClick>()
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
            .add_systems(Update, update_blocks_from_server_on_move)
            .add_systems(Update, display_blockchain_block_details);
        // .add_systems(
        //     OnExit(GameState::PlayerSetup),
        //     despawn_screen::<PlayerSelectMenu>,
        // );
    }
}
