use crate::{resources};
use bevy::app::{App, Plugin};
use bevy::math::{ Vec3};
use bevy::prelude::{Commands, Component, IntoScheduleConfigs, Res, Sprite, Startup, Timer, Transform, Update};
use bevy::sprite::Anchor;
use bevy::time::TimerMode;
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::{LockedAxes, RigidBody};
use crate::resources::GameResources;

pub mod movement;
pub mod shooting;

pub(crate) const MOVEMENT_SPEED: f32 = 300.0;
pub(crate) const PLAYER_ROTATION_SPEED: f32 = 2.1;
pub(crate) const TURRET_ROTATION_SPEED: f32 = 3.1;
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
        app.add_systems(Startup, setup_player.after(resources::load_resources))
            .add_systems(Update, movement::move_sys)
            .add_systems(Update, movement::handle_rotations_by_mouse)
            .add_systems(Update, shooting::fire_bullet);
    }
}
fn setup_player(mut commands: Commands, game_resources: Res<GameResources>) {
    let mut sprite = Sprite::from_image(game_resources.game_atlas.clone());
    sprite.rect = Some(game_resources.tank_body_atlas_rect);
    sprite.flip_y = true;

    let parent = commands
        .spawn((
            Player {
                rotation_speed: PLAYER_ROTATION_SPEED,
                movement_speed: MOVEMENT_SPEED,
            },
            LockedAxes::ROTATION_LOCKED,
            Velocity::default(),
            RigidBody::Dynamic,
            Collider::cuboid(37.5, 35.0), // half-extents of the sprite rect
            sprite,
            Transform::from_translation(Vec3::new(0f32, 0f32, 1f32)),
        ))
        .id();
    let mut turret_sprite = Sprite::from_image(game_resources.game_atlas.clone());
    turret_sprite.rect = Some(game_resources.turret_atlas_rect);
    let turret = commands
        .spawn((
            Turret {
                rotation_speed: TURRET_ROTATION_SPEED,
                firing_timer: Timer::from_seconds(0.10,TimerMode::Once)
            },
            Anchor::BOTTOM_CENTER,
            turret_sprite,
            Transform::from_translation(Vec3::new(0f32, 0f32, 2f32)),
        ))
        .id();

    commands.entity(parent).add_child(turret);
}
