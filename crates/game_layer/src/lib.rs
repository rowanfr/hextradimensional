use bevy::prelude::*;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
    }
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone)]
pub struct HexagonLayer;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone)]
pub struct VoxelLayer;

impl ComputedStates for HexagonLayer {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::Hexagon => Some(HexagonLayer),
            _ => None
        }
    }
}

impl ComputedStates for VoxelLayer {
    type SourceStates = GameState;
    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::Voxel => Some(VoxelLayer),
            _ => None
        }
    }
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    Menu,
    #[default]
    Hexagon,
    Voxel
}

