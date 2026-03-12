use bevy::math::Vec3;
use bevy::prelude::{Component, Query, Res, Time, Transform};

#[derive(Component)]
pub(crate) struct Bullet {
    pub(crate) velocity: Vec3,
}
pub(crate) fn move_bullets(mut bullets: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in &mut bullets {
        transform.translation += bullet.velocity * time.delta_secs();
    }
}