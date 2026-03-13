pub(crate) mod tiles;
pub(crate) mod wall;
pub(crate) mod map_loader;

use crate::resources::GameResources;
use bevy::prelude::*;
use bevy::sprite_render::Material2dPlugin;
use map_loader::{MapAsset, MapAssetLoader};
use wall::{WallMaterial, spawn_wall, sync_wall_materials, WALL_SIZE};
use crate::world::tiles::TileType;

const TILE_SIZE: Vec2 = WALL_SIZE;

#[derive(Resource)]
pub(crate) struct MapHandle(pub(crate) Handle<MapAsset>);

pub(crate) struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        //registers the material with bevy
        app.add_plugins(Material2dPlugin::<WallMaterial>::default())
            //regisiters the asset as a known asset type so we can query it later
            //kinda like a spring bean regersration
            .init_asset::<MapAsset>()
            //registers the loader, the loader declares that it loads the extension ron in its extensions method
            .init_asset_loader::<MapAssetLoader>()
            .add_systems(Startup, load_map.after(crate::resources::load_resources))
            .add_systems(Update, spawn_map_when_ready)
            //the bridge to translate gameplay state to the material
            .add_systems(Update, sync_wall_materials);
    }
}

fn load_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    //since my loaders loads the ron extension, here when i call load bevy sees the ron ext in the file
    //and find the regsited loader mapAssetLoader and uses it. kinda like a custome jackson desrlizer in spring.

    let handle = asset_server.load("maps/level_01.ron");
    commands.insert_resource(MapHandle(handle));
}

// this system will be called every frame and it early returns when the map assets are not ready yet.
//and when its already spawned the map, we determine if it had already spawned the map by decalring the local bool
//this local bool presists accross system calls.
fn spawn_map_when_ready(
    mut commands: Commands,
    map_handle: Res<MapHandle>,
    map_assets: Res<Assets<MapAsset>>,
    game_resources: Res<GameResources>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WallMaterial>>,
    mut already_spawned: Local<bool>,
) {
    if *already_spawned {
        return;
    }
    // this is the let-else pattern. means:
    //try to desctrutre this, and if it doesnt match run the else block. the else block must diverge the function, it cant let it continue
    //meaning it must return, break, continue or panic. because continuing would lead to undefined behavioud as Some(map) is not actually descrtured
    let Some(map) = map_assets.get(&map_handle.0) else {
        return; // not ready yet, try next frame
    };

    let crack_mask = asset_server.load("images/tank/wall_cracks_mask.png");
    let map_width = map.tiles[0].len() as f32 * TILE_SIZE.x;
    let map_height = map.tiles.len() as f32 * TILE_SIZE.y;
    let origin = Vec2::new(-map_width / 2.0, map_height / 2.0);

    for (row, line) in map.tiles.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let world_pos = origin + Vec2::new(
                col as f32 * TILE_SIZE.x + TILE_SIZE.x * 0.5,
                -(row as f32 * TILE_SIZE.y + TILE_SIZE.y * 0.5),
            );

            match TileType::from_char(ch) {
                Some(TileType::Wall) => {
                    spawn_wall(
                        &mut commands,
                        &game_resources,
                        &crack_mask,
                        &mut meshes,
                        &mut materials,
                        world_pos,
                    );
                }
                Some(TileType::PlayerSpawn) => {

                    crate::player::spawn_player(&mut commands, &game_resources, world_pos.extend(1.0));
                }
                _ => {} // Empty, future tile types handled here
            }
        }
    }

    *already_spawned = true;
}

