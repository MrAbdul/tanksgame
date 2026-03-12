use bevy::app::App;
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Handle, Image, Plugin, Res, Resource, Startup};

#[derive(Resource)]
pub(crate) struct GameResources{
    pub(crate) game_atlas: Handle<Image>,
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
    })
}