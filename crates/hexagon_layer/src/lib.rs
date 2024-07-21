mod cells;
mod bundle;
mod chunks;
mod cursor;

pub mod prelude {
    pub use crate::cells::HexId;
    pub use crate::bundle::HexCellBundle;
}

use bevy::prelude::*;

pub struct HexPlugin;

const SEED: [u8; 32] = [
    0b01000010, 0b01100101, 0b01110110, 0b01111001,
    0b01001010, 0b01100001, 0b01101101, 0b00110101,
    0b01101000, 0b01100101, 0b01111000, 0b01110100,
    0b01110010, 0b01100001, 0b01100100, 0b01101001,
    0b01101101, 0b01100101, 0b01101110, 0b01110011,
    0b01101001, 0b01101111, 0b01101110, 0b01100001,
    0b01101100, 0, 0, 0, 0, 0, 0, 0
];

impl Plugin for HexPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, cells::update_transforms);
        app.add_plugins(cursor::CursorPlugin)
        .init_resource::<CellIcons>();
        #[cfg(debug_assertions)]
        // todo Remove from game
        app
        .add_systems(OnEnter(HexagonLayer), tests::spawn_test_grid)
        .add_systems(Update, tests::go_to_voxel.run_if(in_state(HexagonLayer)));

    }
}

mod tests {
    use crate::{cells::{self, CellIcons, HexId, HexNeighbors, HexagonType}, cursor, prelude::HexCellBundle, SEED};
    use bevy::prelude::*;
    use game_layer::{ChangeLayer, HexagonLayer};
    use rand::{Rng, SeedableRng, seq::IteratorRandom};
    use strum::IntoEnumIterator;

    pub fn spawn_test_grid(
        mut commands: Commands,
        icons: Res<CellIcons>,
    ) {
        let mut rng = rand::rngs::StdRng::from_seed(SEED);
        for hex_id in cells::SpiralIter::new(10) {
            let hex_type = if rng.gen_bool(0.1) {
                HexagonType::iter().choose(&mut rng).expect("Iter not Empty")
            } else {
                crate::cells::HexagonType::Empty
            };
            commands.spawn((
                StateScoped(HexagonLayer),
                hex_type,
                HexCellBundle {
                id: hex_id,
                transform: Transform::from_translation(Vec3::NEG_Z * 10.),
                texture: icons.get(hex_type),
                ..Default::default()
            }));
        }
    }

    pub fn go_to_voxel(
        input: Res<ButtonInput<KeyCode>>,
        cursor: Query<(&HexId, &HexNeighbors), With<cursor::Cursor>>,
        hexs: Query<(&HexId, &HexagonType)>,
        mut send: EventWriter<ChangeLayer>
    ) {
        if input.just_pressed(KeyCode::Enter) {
            let cursor = cursor.single();
            let mut hex_type = HexagonType::Empty;
            for (id, hex) in &hexs {
                if id == cursor.0 {
                    hex_type = *hex;
                    break;
                }
            }
            send.send(ChangeLayer::ToVoxel { id: Vec2::new(cursor.0.x(), cursor.0.y()), direction: *cursor.1 as u8, hex_type: hex_type as u8});
        }
    }
}

use cells::CellIcons;
pub use constants::*;
use game_layer::HexagonLayer;
mod constants {
    pub const SQR_3: f32 = 1.732050807568877;
    pub const SQR_3_DIV_TWO: f32 = 0.8660254037844386;
    pub const SQR_3_DIV_THREE: f32 = 0.5773502691896258;
    pub const HEX_SIZE: f32 = 100.;
    pub const HEX_SPACING: f32 = HEX_SIZE / 2.;
}