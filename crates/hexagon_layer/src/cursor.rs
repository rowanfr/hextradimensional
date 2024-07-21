#[derive(Component)]
pub struct Cursor;

use bevy::prelude::*;
use game_layer::{GameState, HexagonLayer, Player};
use strum::IntoEnumIterator;

use crate::{cells::{HexId, HexNeighbors}, HEX_SIZE};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(HexagonLayer), spawn_cursor);
        app.add_systems(OnExit(HexagonLayer), clear_cursor);
        app.add_systems(Update, move_cursor.run_if(in_state(HexagonLayer)));
    }
}

fn spawn_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        HexNeighbors::Direction1,
        Name::new("Cursor"),
        HexId::new(0,0),
        Cursor,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(HEX_SIZE)),
                ..Default::default()
            },
            texture: asset_server.load("images/hexs/outline.png"),
            transform: Transform::from_translation(Vec3::NEG_Z * -9.),
            ..Default::default()
        }
    ));
}

fn clear_cursor(
    mut commands: Commands,
    cursor: Query<Entity, With<Cursor>>,
) {
    for cursor in &cursor {
        commands.entity(cursor).despawn_recursive();
    }
}

fn move_cursor(
    player: Query<&Transform, With<Player>>,
    mut cursors: Query<(&mut HexId, &mut Transform, &mut HexNeighbors), (With<Cursor>, Without<Player>)>,
){
    let player = player.single();
    let id = HexId::from_xyz(player.translation);
    for (mut cursor, mut pos, mut n) in &mut cursors {
        if &id != cursor.as_ref() {
            *cursor = id;
        }
        let mut direction = HexNeighbors::Direction1;
        let mut distance = (id + HexNeighbors::Direction1).xyz().distance_squared(player.translation);
        for neighbor in HexNeighbors::iter() {
            let next = ((id + neighbor).xyz() / 2.).distance_squared(player.translation);
            if next < distance {
                distance = next;
                direction = neighbor;
            }
        }
        pos.rotation = Quat::from_rotation_z(direction.angle());
        if &direction != n.as_ref() {
            *n = direction;
        }
    }
}