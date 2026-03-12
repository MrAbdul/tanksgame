use bevy::app::{App, Update};
use bevy::math::Vec3;
use bevy::prelude::{Component, Plugin, Query, Res, Time, Transform};

#[derive(Component)]
pub(crate) struct Bullet {
    pub(crate) velocity: Vec3,
}

pub(crate) struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_bullets);
    }
}
pub(crate) fn move_bullets(mut bullets: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in &mut bullets {
        transform.translation += bullet.velocity * time.delta_secs();
    }
}
