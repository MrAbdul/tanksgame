use bevy::app::App;
use bevy::asset::AssetServer;
use bevy::math::Rect;
use bevy::prelude::{Commands, Handle, Image, Plugin, Res, Resource, Startup};
use crate::effects;

#[derive(Resource)]
pub(crate) struct GameResources{
    pub(crate) game_atlas: Handle<Image>,
    pub(crate) bullet_atlas_rect:Rect,
    pub(crate) tank_body_atlas_rect:Rect,
    pub(crate) turret_atlas_rect:Rect,
    pub(crate) effect_resources:effects::AnimationAssets
}

pub(crate) struct GameResourcesPlugin;

impl Plugin for GameResourcesPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,load_resources);
    }
}

pub(crate) fn load_resources(mut commands: Commands,asset_server: Res<AssetServer>){
    commands.insert_resource(GameResources{
        game_atlas: asset_server.load("images/tank/sheet_tanks.png"),
        bullet_atlas_rect:Rect::new(148.0, 345.0, 148.0 + 20.0, 345.0 + 33.0),
        tank_body_atlas_rect:Rect::new(671.0, 70.0, 746.0, 140.0),
        turret_atlas_rect:Rect::new(850.0, 58.0, 850.0 + 16.0, 58.0 + 50.0),
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
    }})

}