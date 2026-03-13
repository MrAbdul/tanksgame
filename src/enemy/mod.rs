use bevy::app::App;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, Plugin, Res, Sprite, TimerMode, Transform, Update};
use bevy::sprite::Anchor;
use bevy::time::Timer;
use bevy_rapier2d::dynamics::{LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::Collider;
use crate::resources::GameResources;

pub mod ai;
pub(crate) const MOVEMENT_SPEED: f32 = 300.0;
pub(crate) const PLAYER_ROTATION_SPEED: f32 = 2.1;
pub(crate) const TURRET_ROTATION_SPEED: f32 = 0.5;

#[derive(Component)]
pub(crate) struct  Enemy{
    pub(crate) movement_speed: f32,
    pub(crate) rotation_speed: f32,
}

#[derive(Component)]
pub(crate) struct EnemyTurret{
    pub(crate) rotation_speed: f32,
    pub(crate) firing_timer: Timer,
}

pub(crate) struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,ai::rotate_turret_towards_player);
    }
}

pub(crate) fn spawn_enemy(commands: &mut Commands, game_resources: &Res<GameResources>, position: Vec3){
    let mut sprite = Sprite::from_image(game_resources.game_atlas.clone());
    sprite.rect=Some(game_resources.enemy_tank_body_atlas_rect);
    sprite.flip_y=true;
    let parent = commands
        .spawn((
            Enemy {
                rotation_speed: PLAYER_ROTATION_SPEED,
                movement_speed: MOVEMENT_SPEED,
            },
            LockedAxes::ROTATION_LOCKED,
            Velocity::default(),
            RigidBody::Dynamic,
            Collider::cuboid(37.5, 35.0), // half-extents of the sprite rect
            sprite,
            Transform::from_translation(position),
        ))
        .id();
    let mut turret_sprite = Sprite::from_image(game_resources.game_atlas.clone());
    turret_sprite.rect = Some(game_resources.enemy_turret_atlas_rect);
    let turret = commands
        .spawn((
            EnemyTurret {
                rotation_speed: TURRET_ROTATION_SPEED,
                firing_timer: Timer::from_seconds(0.3,TimerMode::Once)
            },
            Anchor::BOTTOM_CENTER,
            turret_sprite,
            Transform::from_translation(Vec3::new(0f32, 0f32, 2f32)),
        ))
        .id();

    commands.entity(parent).add_child(turret);

}