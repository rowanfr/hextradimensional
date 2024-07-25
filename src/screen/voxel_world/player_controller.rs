use crate::screen::{
    voxel_world::voxel_util::{Solid, VoxelPlayer},
    Screen,
};
use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct VoxelCamera;

impl Plugin for VoxelCamera {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoxelSettings>()
            .init_resource::<InputState>()
            .add_systems(Update, cursor_grab)
            .add_systems(Update, (player_look, player_move, apply_jump, player_jump))
            .add_systems(OnEnter(Screen::VoxelWorld), initial_grab_cursor);
    }
}

fn player_move(
    mut player: Query<&mut Transform, With<VoxelPlayer>>,
    settings: Res<VoxelSettings>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    solid: Res<Solid>,
) {
    for mut player in &mut player {
        let mut delta = Vec3::ZERO;
        if input.pressed(settings.move_forward) {
            delta.z += 1.;
        }
        if input.pressed(settings.move_backward) {
            delta.z -= 1.;
        }
        if input.pressed(settings.move_left) {
            delta.x -= 1.;
        }
        if input.pressed(settings.move_right) {
            delta.x -= 1.;
        }
        let mut forward = player.forward().as_vec3();
        forward.y = 0.;
        forward = forward.normalize();
        let mut right = player.right().as_vec3();
        right.y = 0.;
        right = right.normalize();
        let mut next = player.translation + forward * delta.z * time.delta_seconds() * 10.;
        next += right * delta.x * time.delta_seconds() * 10.;
        let off = next.round().as_ivec3();
        if !solid.get(off.x, off.y, off.z) {
            player.translation = next;
        }
    }
}

// don't know why this is here maybe legacy from the flycam im copying
#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

fn player_look(
    settings: Res<VoxelSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<VoxelPlayer>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.read(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -=
                            (settings.mouse_sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -=
                            (settings.mouse_sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<VoxelSettings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(key_bindings.toggle_grab_cursor) {
            match window.cursor.grab_mode {
                CursorGrabMode::None => {
                    window.cursor.grab_mode = CursorGrabMode::Confined;
                    window.cursor.visible = false;
                }
                _ => {
                    window.cursor.grab_mode = CursorGrabMode::None;
                    window.cursor.visible = true;
                }
            }
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

fn initial_grab_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        match window.cursor.grab_mode {
            CursorGrabMode::None => {
                window.cursor.grab_mode = CursorGrabMode::Confined;
                window.cursor.visible = false;
            }
            _ => {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            }
        }
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

#[derive(Resource)]
pub struct VoxelSettings {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub jump: KeyCode,
    pub toggle_grab_cursor: KeyCode,
    pub move_speed: f32,
    pub mouse_sensitivity: f32,
}

impl Default for VoxelSettings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            jump: KeyCode::Space,
            toggle_grab_cursor: KeyCode::Backquote,
            mouse_sensitivity: 0.00012,
            move_speed: 12.,
        }
    }
}

#[derive(Component)]
struct Jump {
    left: f32,
}

// jumping does not disable gravity so a value less then 9.8 will not make you move up
const JUMP_POWER: f32 = 9.8 * 5.;

fn apply_jump(
    mut commands: Commands,
    mut jumping: Query<(Entity, &mut Jump, &mut Transform)>,
    time: Res<Time>,
    solid: Res<Solid>,
) {
    // the max jump for this frame
    let max_use = JUMP_POWER * time.delta_seconds();
    for (entity, mut jump, mut pos) in &mut jumping {
        // get current block
        let block = pos.translation.floor().as_ivec3();
        // get block 1 up
        let up = block + IVec3::Y;
        // if block 1 up is solid cant jump
        if solid.get(up.x, up.y, up.z) {
            jump.left = 0.;
        }
        // the amount this entity is going to move
        let using = max_use.min(jump.left);
        // move entity
        pos.translation.y += using;
        // remove this jump from total
        jump.left -= using;
        // if no jump left remove component
        if jump.left <= 0. {
            commands.entity(entity).remove::<Jump>();
        }
    }
}

fn player_jump(
    input: Res<ButtonInput<KeyCode>>,
    settings: Res<VoxelSettings>,
    mut commands: Commands,
    players: Query<(Entity, &Transform), With<VoxelPlayer>>,
    solid: Res<Solid>,
) {
    if input.just_pressed(settings.jump) {
        for (entity, player) in &players {
            if player.translation.y.fract() > 0.2 {
                continue;
            }
            info!("Adding Jump");
            let block = player.translation.floor().as_ivec3();
            let down = block - IVec3::Y;
            if solid.get(down.x, down.y, down.z) {
                commands.entity(entity).insert(Jump { left: 1.2 });
            }
        }
    }
}
