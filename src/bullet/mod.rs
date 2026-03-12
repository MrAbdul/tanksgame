use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::time::Timer;
use crate::effects;

#[derive(Component)]
pub(crate) struct Bullet {
    pub(crate) lifetime: Timer,
}

pub(crate) struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Update, move_bullets)
            .add_systems(Update,despawn_bullets)
            .add_systems(Update,bullet_hit_wall);
    }
}
// pub(crate) fn move_bullets(mut bullets: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
//     for (mut transform, bullet) in &mut bullets {
//         transform.translation += bullet.velocity * time.delta_secs();
//     }
// }
fn despawn_bullets(mut commands: Commands, mut bullets: Query<(Entity, &mut Bullet, &Transform)>, time: Res<Time>,smoke_assets:Res<effects::AnimationAssets> ) {
    for (entity, mut bullet, transform) in &mut bullets {
        bullet.lifetime.tick(time.delta());
        if bullet.lifetime.is_finished() {
            commands.spawn(effects::SmokeEffect::new(
                effects::SmokeType::Yellow,
                &smoke_assets,
                transform.translation,
                0.05,
            ));
            commands.entity(entity).despawn();
        }
    }
}
fn bullet_hit_wall(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
    bullets: Query<Entity, With<Bullet>>,
    walls: Query<Entity, With<crate::world::Wall>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let bullet_entity =
                if bullets.contains(*e1) && walls.contains(*e2) {
                    *e1
                } else if bullets.contains(*e2) && walls.contains(*e1) {
                    *e2
                } else {
                    continue;
                };

            println!("bullet hit wall");
            commands.entity(bullet_entity).despawn();
        }
    }
}