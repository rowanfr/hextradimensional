//! Game mechanics and content.

use bevy::prelude::*;
use game_layer::GameState;

use crate::screen::Screen;

mod animation;
pub mod assets;
pub mod audio;
mod movement;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_gamestate.run_if(state_changed::<Screen>));
    app.add_plugins((
        animation::plugin,
        audio::plugin,
        assets::plugin,
        movement::plugin,
        spawn::plugin,
        game_layer::GamePlugin,
        hexagon_layer::HexPlugin,
        voxel_layer::VoxelPlugin,
    ));
}

// todo Remove from game
fn update_gamestate(
    template_state: Res<State<Screen>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    match template_state.get() {
        Screen::Playing => game_state.set(GameState::Hexagon),
        _ => game_state.set(GameState::Menu),
    }
}
 