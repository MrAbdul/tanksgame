use crate::{resources};
use bevy::app::{App, Plugin};
use bevy::asset::{ Handle};
use bevy::image::Image;
use bevy::math::{Rect, Vec3};
use bevy::prelude::{Commands, Component, IntoScheduleConfigs, Res, Resource, Sprite, Startup, Timer, Transform, Update};
use bevy::sprite::Anchor;
use bevy::time::TimerMode;
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

#[derive(Resource)]
pub(crate) struct TankResources {
    pub(crate) image: Handle<Image>,
    pub(crate) body: Rect,
    pub(crate) turret: Rect,
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
    let tank_body_rect = Rect::new(671.0, 70.0, 746.0, 140.0);
    let turret_rect = Rect::new(850.0, 58.0, 850.0 + 16.0, 58.0 + 50.0);
    let tank_resources = TankResources {
        image: game_resources.game_atlas.clone(),
        body: tank_body_rect,
        turret: turret_rect,
    };

    let mut sprite = Sprite::from_image(tank_resources.image.clone());
    sprite.rect = Some(tank_resources.body);
    sprite.flip_y = true;

    let parent = commands
        .spawn((
            Player {
                rotation_speed: PLAYER_ROTATION_SPEED,
                movement_speed: MOVEMENT_SPEED,
            },
            sprite,
            Transform::from_translation(Vec3::new(0f32, 0f32, 1f32)),
        ))
        .id();
    let mut turret_sprite = Sprite::from_image(game_resources.game_atlas.clone());
    turret_sprite.rect = Some(tank_resources.turret);
    let turret = commands
        .spawn((
            Turret {
                rotation_speed: TURRET_ROTATION_SPEED,
                firing_timer: Timer::from_seconds(1.0,TimerMode::Once)
            },
            Anchor::BOTTOM_CENTER,
            turret_sprite,
            Transform::from_translation(Vec3::new(0f32, 0f32, 2f32)),
        ))
        .id();

    commands.entity(parent).add_child(turret);
    commands.insert_resource(tank_resources);
}
