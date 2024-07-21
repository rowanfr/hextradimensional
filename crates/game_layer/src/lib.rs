use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<GameState>()
        .add_computed_state::<HexagonLayer>()
        .add_computed_state::<VoxelLayer>()
        .add_systems(PreUpdate, change_layer)
        .enable_state_scoped_entities::<HexagonLayer>()
        .enable_state_scoped_entities::<VoxelLayer>()
        .add_event::<ChangeLayer>();
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
    #[default]
    Menu,
    Hexagon,
    Voxel
}

fn change_layer(
    mut events: EventReader<ChangeLayer>,
    mut next: ResMut<NextState<GameState>>,
) {
    for event in events.read() {
        match event {
            ChangeLayer::ToHex { id, direction } => next.set(GameState::Hexagon),
            ChangeLayer::ToVoxel { id, direction, hex_type} => next.set(GameState::Voxel),
        }
    }
}

#[derive(Event)]
pub enum ChangeLayer {
    ToHex {
        id: Vec2,
        direction: u8,
    },
    ToVoxel {
        id: Vec2,
        direction: u8,
        hex_type: u8,
    }
}
