use bevy::app::App;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, Component, Plugin, Res, Sprite, TimerMode, Transform, Update};
use bevy::sprite::Anchor;
use bevy::time::Timer;
use bevy_rapier2d::dynamics::{LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::Collider;
use crate::{bullet, health};
use crate::resources::{GameConfig, GameResources};

pub mod ai;

#[derive(Component)]
pub(crate) struct  Enemy{
    pub(crate) movement_speed: f32,
    pub(crate) rotation_speed: f32,
    pub(crate) stuck_timer: f32,
    pub(crate) previous_distance_to_player:f32
    ,

}

#[derive(Component)]
pub(crate) struct EnemyTurret{
    pub(crate) rotation_speed: f32,
    pub(crate) firing_timer: Timer,
}

pub(crate) struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,ai::rotate_turret_towards_player)
            .add_systems(Update,ai::move_towards_player);
    }
}

pub(crate) fn spawn_enemy(commands: &mut Commands, game_resources: &Res<GameResources>, position: Vec3,game_config: &Res<GameConfig>){
    let mut sprite = Sprite::from_image(game_resources.game_atlas.clone());
    sprite.rect=Some(game_resources.enemy_tank_body_atlas_rect);
    sprite.flip_y=true;
    let parent = commands
        .spawn((
            Enemy {
                rotation_speed: game_config.enemy_tank_rotation_speed,
                movement_speed: game_config.enemy_tank_movement_speed,
                stuck_timer:0.0,
                previous_distance_to_player:0.0,
            },
            health::HealthBundle{
                health:health::Health{
                    health: game_config.enemy_tank_health
                },
                damaged_bullets:health::TakesDamageFrom{
                    damaging_bullets:vec![bullet::BulletType::Blue]
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
    turret_sprite.rect = Some(game_resources.enemy_turret_atlas_rect);
    let turret = commands
        .spawn((
            EnemyTurret {
                rotation_speed: game_config.enemy_turret_rotation_speed,
                firing_timer: Timer::from_seconds(game_config.enemy_firing_timer,TimerMode::Once)
            },
            Anchor::BOTTOM_CENTER,
            turret_sprite,
            Transform::from_translation(Vec3::new(0f32, 0f32, 2f32)),
        ))
        .id();

    commands.entity(parent).add_child(turret);

}