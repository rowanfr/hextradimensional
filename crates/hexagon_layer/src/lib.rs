mod cells;
mod bundle;
mod chunks;

pub mod prelude {
    pub use crate::cells::HexId;
    pub use crate::bundle::HexCellBundle;
}

use bevy::prelude::*;

pub struct HexPlugin;

impl Plugin for HexPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, cells::update_transforms);
        #[cfg(debug_assertions)]
        // todo Remove from game
        app.add_systems(Update, tests::spawn_test_grid);
    }
}

mod tests {
    use crate::{cells, prelude::HexCellBundle};
    use bevy::prelude::*;

    pub fn spawn_test_grid(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        let hex = asset_server.load("images/hexs/blank.png");
        for hex_id in cells::SpiralIter::new(4) {
            commands.spawn((HexCellBundle {
                id: hex_id,
                transform: Transform::from_translation(Vec3::NEG_Z * 10.),
                texture: hex.clone(),
                ..Default::default()
            }));
        }
    }
}

pub use constants::*;
mod constants {
    pub const SQR_3: f32 = 1.732050807568877;
    pub const SQR_3_DIV_TWO: f32 = 0.8660254037844386;
    pub const HEX_SIZE: f32 = 100.;
    pub const HEX_SPACING: f32 = HEX_SIZE / 2.;
}