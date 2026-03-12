pub mod bullet;
pub mod effects;
pub mod player;
pub mod resources;

use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_plugins(resources::GameResourcesPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(bullet::BulletPlugin)
        .add_plugins(effects::EffectsPlugin)
        .run()
}

//startup system
fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);

}
