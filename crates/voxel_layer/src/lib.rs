use bevy::{prelude::*, utils::HashMap};
use game_layer::{ChangeLayer, VoxelLayer};
use rand::{Rng, SeedableRng};
use strum::IntoEnumIterator;

mod player_controler;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(VoxelLayer), spawn_voxel_map);
        app.add_systems(Update, (gravity, back_to_hex).in_set(VoxelLayerSystems::Update));
        app.init_resource::<Solid>();
        app.init_resource::<Blocks>()
        .configure_sets(Update, VoxelLayerSystems::Update.run_if(in_state(VoxelLayer)))
        .add_plugins(player_controler::VoxelCamera);
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum VoxelLayerSystems {
    Update,
}

#[derive(Component)]
struct VoxelPlayer;

fn spawn_voxel_map(
    mut layer_change: EventReader<ChangeLayer>,
    mut commands: Commands,
    blocks: Res<Blocks>,
    mut solid: ResMut<Solid>,
) {
    let Some(ChangeLayer::ToVoxel {id, direction, hex_type}) = layer_change.read().last() else {return;};
    commands.spawn((
        VoxelPlayer,
        Gravity,
        StateScoped(VoxelLayer),
        Camera3dBundle {
            camera: Camera {
                order: 1,
                ..Default::default()
            },
            transform: Transform::from_translation(pos_from_enter(*direction)),
            ..Default::default()
        }
    ));
    commands.insert_resource(ChunkId(*id));
    fill_world(
        commands,
        *id,
        WorldType::from_u8(*hex_type),
        blocks.as_ref(),
        &mut solid,
    );
}

fn pos_from_enter(direction: u8) -> Vec3 {
    match direction {
        0 => Vec3::new(8., 0.,8.),
        1 => Vec3::new(16., 8., 8.),
        2 => Vec3::new(8., 8., 16.),
        3 => Vec3::new(8., 16., 8.),
        4 => Vec3::new(0., 8., 8.),
        5 => Vec3::new(8., 8., 0.),
        _ => unreachable!()
    }
}

#[derive(Resource)]
struct ChunkId(Vec2);

#[derive(Component)]
struct Gravity;

fn gravity(
    solid: Res<Solid>,
    mut player: Query<&mut Transform, With<Gravity>>,
    time: Res<Time>,
) {
    for mut player in &mut player {
        let block = player.translation.floor().as_ivec3();
        if !solid.get(block.x, block.y - 1, block.z) {
            player.translation -= Vec3::Y * 9.8 * time.delta_seconds()
        }
    }
}

fn back_to_hex(
    player: Query<&Transform, With<VoxelPlayer>>,
    mut events: EventWriter<ChangeLayer>,
    chunk: Res<ChunkId>,
) {
    for player in &player {
        if player.translation.y < -2. {
            events.send(ChangeLayer::ToHex { id: chunk.0, direction: 3 });
        } else if player.translation.y > 18. {
            events.send(ChangeLayer::ToHex { id: chunk.0, direction: 0 });
        } else if player.translation.x < -2. {
            events.send(ChangeLayer::ToHex { id: chunk.0, direction: 4 });
        } else if player.translation.x > 18. {
            events.send(ChangeLayer::ToHex { id: chunk.0, direction: 1 });
        } else if player.translation.z < -2. {
            events.send(ChangeLayer::ToHex { id: chunk.0, direction: 5 });
        } else if player.translation.z > 18. {
            events.send(ChangeLayer::ToHex { id: chunk.0, direction: 2 });
        }
    }
}

#[derive(PartialEq, Eq)]
enum WorldType {
    Empty,
    Stone,
    Coal,
}

#[derive(Resource)]
struct Solid([bool; 16*16*16]);

impl Default for Solid {
    fn default() -> Self {
        Self([false; 16 * 16 * 16])
    }
}

impl Solid {
    fn set(&mut self, x: i32, y: i32, z: i32, val: bool) {
        self.0[(x + z * 16 + y * 16 * 16) as usize] = val;
    }
    fn clear(&mut self) {
        self.0 = [false; 16*16*16];
    }
    fn get(&self, x: i32, y: i32, z: i32) -> bool {
        self.0.get((x + z * 16 + y * 16 * 16) as usize).cloned().unwrap_or(false)
    }
}

impl WorldType {
    fn from_u8(id: u8) -> WorldType {
        match id {
            0 => WorldType::Empty,
            1 => WorldType::Stone,
            2 => WorldType::Coal,
            _ => unreachable!()
        }
    }

    fn sample(&self, mut rng: impl Rng, pos: IVec3) -> BlockType {
        match self {
            WorldType::Empty => BlockType::Air,
            WorldType::Stone => {
                if rng.gen_bool(0.6) || pos.y == 0 {
                    BlockType::Stone
                } else {
                    BlockType::Air
                }
            },
            WorldType::Coal => {
                if rng.gen_bool(0.3) && pos.y != 0 {
                    BlockType::Air
                } else if rng.gen_bool(0.25) {
                    BlockType::Coal
                } else {
                    BlockType::Stone
                }
            },
        }
    }
}

fn fill_world(
    mut commands: Commands,
    id: Vec2,
    world_type: WorldType,
    blocks: &Blocks,
    solid: &mut Solid,
) {
    solid.clear();
    if world_type == WorldType::Empty {return;}
    let mut rng = rand::rngs::StdRng::seed_from_u64((id.x as u64) << 32 | id.y as u64); 
    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let block = world_type.sample(&mut rng, IVec3::new(x, y, z));
                solid.set(x, y, z, block.is_solid());
                commands.spawn((
                    StateScoped(VoxelLayer),
                    PbrBundle {
                        mesh: blocks.mesh(),
                        material: blocks.texture(block),
                        transform: Transform::from_translation(Vec3::new(x as f32, y as f32, z as f32)),
                        ..Default::default()
                    }
                ));
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, strum_macros::EnumIter)]
enum BlockType {
    Air,
    Stone,
    Coal,
}

impl BlockType {
    const fn texture_path(&self) -> &'static str {
        match self {
            BlockType::Air => "",
            BlockType::Stone => "images/voxels/stone.png",
            BlockType::Coal => "images/voxels/coal.png",
        }
    }

    const fn is_solid(&self) -> bool {
        match self {
            BlockType::Air => false,
            BlockType::Stone => true,
            BlockType::Coal => true,
        }
    }
}

#[derive(Resource)]
struct Blocks{
    mesh: Handle<Mesh>,
    textures: HashMap<BlockType, Handle<StandardMaterial>>
}

impl Blocks {
    pub fn texture(&self, block: BlockType) -> Handle<StandardMaterial> {
        self.textures.get(&block).cloned().unwrap_or_default()
    }
    pub fn mesh(&self) -> Handle<Mesh> {
        self.mesh.clone()
    }
}

impl FromWorld for Blocks {
    fn from_world(world: &mut World) -> Self {
        let mut blocks = Blocks {
            mesh: world.resource_mut::<Assets<Mesh>>().add(Cuboid::new(1., 1., 1.)),
            textures: HashMap::default(),
        };
        let asset_server = world.resource::<AssetServer>().clone();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        for block in BlockType::iter() {
            blocks.textures.insert(block, materials.add(
                StandardMaterial {
                    base_color_texture: Some(asset_server.load(block.texture_path())),
                    ..Default::default()
                }
            ));
        }

        blocks
    }
}