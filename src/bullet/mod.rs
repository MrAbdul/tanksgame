use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::time::Timer;
use crate::{effects, PendingDespawn};
use crate::resources::GameResources;

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
            .add_systems(Update,bullet_hit_wall.after(despawn_bullets));
    }
}
// pub(crate) fn move_bullets(mut bullets: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
//     for (mut transform, bullet) in &mut bullets {
//         transform.translation += bullet.velocity * time.delta_secs();
//     }
// }
fn despawn_bullets(mut commands: Commands, mut bullets: Query<(Entity, &mut Bullet, &Transform),Without<PendingDespawn>>, time: Res<Time>,game_resources:Res<GameResources> ) {
    for (entity, mut bullet, transform) in &mut bullets {
        bullet.lifetime.tick(time.delta());
        if bullet.lifetime.is_finished() {
            commands.spawn(effects::SmokeEffect::new(
                effects::SmokeType::Yellow,
                &game_resources.effect_resources,
                transform.translation,
                0.05,
            ));
            commands.entity(entity).insert(PendingDespawn);
        }
    }
}
pub(crate) fn bullet_hit_wall(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
    bullets: Query<Entity, (With<Bullet>, Without<PendingDespawn>)>, // add this
    mut walls: Query<(&mut crate::world::Wall, &mut crate::world::WallFlash), Without<PendingDespawn>>, // add this
)  {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            if bullets.contains(*e1) {
                if let Ok((mut wall, mut flash)) = walls.get_mut(*e2) {
                    wall.health = (wall.health - 25.0).max(0.0);
                    flash.timer = Timer::from_seconds(0.05, TimerMode::Once);
                    if wall.health==0.0 {
                        commands.entity(*e2).insert(PendingDespawn);
                    }
                    commands.entity(*e1).insert(PendingDespawn);
                }
            } else if bullets.contains(*e2) {
                if let Ok((mut wall, mut flash)) = walls.get_mut(*e1) {
                    wall.health = (wall.health - 25.0).max(0.0);
                    flash.timer = Timer::from_seconds(0.05, TimerMode::Once);
                    if wall.health==0.0 {
                        commands.entity(*e1).insert(PendingDespawn);
                    }
                    commands.entity(*e2).insert(PendingDespawn);
                }
            }
        }
    }
}