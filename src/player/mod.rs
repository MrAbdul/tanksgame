use bevy::app::{App, Plugin};
use bevy::math::{ Vec3};
use bevy::prelude::{Commands, Component, Res, Sprite, Timer, Transform, Update};
use bevy::sprite::Anchor;
use bevy::time::TimerMode;
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::{LockedAxes, RigidBody};
use crate::{bullet, health};
use crate::resources::{GameConfig, GameResources};

pub mod movement;
pub mod shooting;

#[derive(Component)]
pub(crate) struct Player {
    pub(crate) movement_speed: f32,
    pub(crate) rotation_speed: f32,
}
#[derive(Component)]
pub(crate) struct Turret {
    pub(crate) rotation_speed: f32,
    pub(crate) firing_timer: Timer,
}

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, movement::move_sys)
            .add_systems(Update, movement::handle_rotations_by_mouse)
            .add_systems(Update, shooting::fire_bullet)
            .add_systems(Update, movement::camera_follow);
    }
}
pub(crate) fn spawn_player( commands: &mut Commands, game_resources: &Res<GameResources>,position:Vec3,game_config:&Res<GameConfig> ) {
    let mut sprite = Sprite::from_image(game_resources.game_atlas.clone());
    sprite.rect = Some(game_resources.tank_body_atlas_rect);
    sprite.flip_y = true;

    let parent = commands
        .spawn((
            Player {
                rotation_speed: game_config.player_tank_rotation_speed,
                movement_speed: game_config.player_tank_movement_speed,
            },
            health::HealthBundle{
                health:health::Health{
                    health: game_config.player_tank_health
                },
                damaged_bullets:health::TakesDamageFrom{
                    damaging_bullets:vec![bullet::BulletType::Red]
                }
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
    turret_sprite.rect = Some(game_resources.turret_atlas_rect);
    let turret = commands
        .spawn((
            Turret {
                rotation_speed: game_config.player_turret_rotation_speed,
                firing_timer: Timer::from_seconds(game_config.player_firing_timer,TimerMode::Once)
            },
            Anchor::BOTTOM_CENTER,
            turret_sprite,
            Transform::from_translation(Vec3::new(0f32, 0f32, 2f32)),
        ))
        .id();

    commands.entity(parent).add_child(turret);
}
