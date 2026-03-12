use bevy::asset::Handle;
use bevy::image::Image;
use bevy::math::{Rect,};
use bevy::prelude::{Component, Resource};

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
}


#[derive(Resource)]
pub(crate) struct TankResources {
    pub(crate) image: Handle<Image>,
    pub(crate) body: Rect,
    pub(crate) turret: Rect,
}