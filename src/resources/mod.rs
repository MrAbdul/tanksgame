pub mod game_config_loader;

use bevy::app::App;
use bevy::asset::AssetServer;
use bevy::audio::AudioSource;
use bevy::math::Rect;
use bevy::prelude::{Asset, AssetApp, Assets, Commands, Handle, Image, Local, Plugin, Res, Resource, Startup, TypePath, Update};
use serde::Deserialize;
use crate::effects;
use crate::resources::game_config_loader::GameConfigAssetLoader;

#[derive(Resource)]
pub(crate) struct GameResources{
    pub(crate) game_atlas: Handle<Image>,
    pub(crate) bullet_atlas_rect:Rect,
    pub(crate) tank_body_atlas_rect:Rect,
    pub(crate) turret_atlas_rect:Rect,
    pub(crate) effect_resources:effects::AnimationAssets,
    pub(crate) enemy_tank_body_atlas_rect: Rect,
    pub(crate) enemy_turret_atlas_rect:Rect,
    pub(crate) bullet_enemy_atlas_rect:Rect,
    pub(crate) sound_firing:Handle<AudioSource>,
}

#[derive(Resource, Asset, TypePath, Deserialize, Debug,Clone)]
pub(crate) struct GameConfig{
    pub player_bullet_base_velocity: f32,//500.0
    pub enemy_bullet_base_velocity: f32,//500.0
    pub muzzle_smoke_effect_frame_duration: f32,//0.1
    pub bullet_explosion_effect_frame_duration: f32,//0.05
    pub bullet_wall_damage_amount: f32,//25.0
    pub wall_hit_flash_duration: f32, //0.05
    pub enemy_tank_rotation_speed: f32, //2.1
    pub enemy_tank_movement_speed: f32, //300.0
    pub enemy_turret_rotation_speed: f32, //0.3
    pub enemy_firing_timer: f32, //0.5
    pub enemy_targeting_angle: f32, //5.0
    pub player_tank_rotation_speed: f32, //2.1
    pub player_tank_movement_speed: f32,//350
    pub player_turret_rotation_speed: f32,//3.1
    pub player_firing_timer: f32, //0.5
    pub player_rotation_lock_radius: f32,//1000.0
    pub camera_smoothing_factor: f32,//0.1
}
#[derive(Resource)]
pub(crate) struct GameConfigHandle(pub(crate) Handle<GameConfig>);
pub(crate) struct GameResourcesPlugin;

impl Plugin for GameResourcesPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_asset::<GameConfig>()
            .init_asset_loader::<GameConfigAssetLoader>()
            .add_systems(Startup,load_resources)
            .add_systems(Startup,load_config)
            .add_systems(Update,promote_config_to_resource);
    }
}
fn load_config(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle: Handle<GameConfig> = asset_server.load("config/game_config.ron");
    commands.insert_resource(GameConfigHandle(handle));
}
pub(crate) fn promote_config_to_resource(
    mut commands: Commands,
    handle: Res<GameConfigHandle>,
    assets: Res<Assets<GameConfig>>,
    mut already_done: Local<bool>,
) {
    if *already_done { return; }
    let Some(config) = assets.get(&handle.0) else { return; };
    commands.insert_resource(config.clone());
    *already_done = true;
}
pub(crate) fn load_resources(mut commands: Commands,asset_server: Res<AssetServer>){
    commands.insert_resource(GameResources{
        game_atlas: asset_server.load("images/tank/sheet_tanks.png"),
        bullet_atlas_rect:Rect::new(148.0, 345.0, 148.0 + 20.0, 345.0 + 33.0),
        tank_body_atlas_rect:Rect::new(671.0, 70.0, 746.0, 140.0),
        turret_atlas_rect:Rect::new(850.0, 58.0, 850.0 + 16.0, 58.0 + 50.0),
        enemy_tank_body_atlas_rect:Rect::new(588.0, 0.0, 588.0 + 83.0, 0.0 + 78.0),
        bullet_enemy_atlas_rect:Rect::new(711.0, 140.0, 711.0 + 20.0, 140.0 + 34.0),
        enemy_turret_atlas_rect:Rect::new(834.0, 0.0, 834.0 + 24.0, 0.0 + 58.0),
        effect_resources: effects::AnimationAssets {
        image: asset_server.load("images/tank/sheet_tanks.png"),
        grey_smoke_frames: vec![
            Rect::new(416.0, 188.0, 416.0 + 87.0, 188.0 + 87.0),
            Rect::new(296.0, 408.0, 296.0 + 92.0, 408.0 + 89.0),
            Rect::new(478.0, 384.0, 478.0 + 90.0, 384.0 + 99.0),
        ],
        yellow_smoke_frames: vec![
            Rect::new(228.0, 107.0, 228.0 + 87.0, 107.0 + 87.0),
            Rect::new(326.0, 0.0, 326.0 + 92.0, 0.0 + 89.0),
            Rect::new(416.0, 89.0, 416.0 + 90.0, 89.0 + 99.0),

        ],
    },

        sound_firing: asset_server.load("sound_effects/tank-firing.ogg"),
    })

}