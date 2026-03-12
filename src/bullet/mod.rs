use bevy::app::{App, Update};
use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, Entity, Plugin, Query, Res, Time, Transform};
use bevy::time::Timer;

#[derive(Component)]
pub(crate) struct Bullet {
    pub(crate) velocity: Vec3,
    pub(crate) lifetime: Timer,
}

pub(crate) struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_bullets)
            .add_systems(Update,despawn_bullets);
    }
}
pub(crate) fn move_bullets(mut bullets: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in &mut bullets {
        transform.translation += bullet.velocity * time.delta_secs();
    }
}
fn despawn_bullets(mut commands: Commands, mut bullets: Query<(Entity, &mut Bullet)>, time: Res<Time> ) {
    for (entity, mut bullet) in &mut bullets {
        bullet.lifetime.tick(time.delta());
        if bullet.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
