pub mod bullet;
pub mod effects;
pub mod player;
pub mod resources;
pub mod world;
pub mod enemy;
pub mod ui;
pub mod audio;

use bevy::prelude::*;
use bevy_rapier2d::math::Vect;
use bevy_rapier2d::plugin::{RapierConfiguration, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin};

#[derive(Component)]
pub(crate) struct PendingDespawn;
fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default()) // remove later
        .add_systems(Startup, (startup, setup_physics))
        .add_plugins(resources::GameResourcesPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(bullet::BulletPlugin)
        .add_plugins(effects::EffectsPlugin)
        .add_plugins(world::WorldPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(audio::GameAudioPlugin)
        //after the systems that will attach despawn
        .add_systems(
            Update,
            cleanup
                .after(bullet::bullet_hit_wall)
                .after(effects::animate_smoke),
        )
        .run()
}

//startup system
fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn setup_physics(mut config: Query<&mut RapierConfiguration>) {
    let mut config = config.single_mut().unwrap();
    config.gravity = Vect::ZERO;
}
fn cleanup(mut commands: Commands, query: Query<Entity, With<PendingDespawn>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
