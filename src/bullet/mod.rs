use crate::resources::GameResources;
use crate::{effects, PendingDespawn};
use bevy::prelude::*;
use bevy::time::Timer;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub(crate) struct Bullet {
    pub(crate) lifetime: Timer,
}

pub(crate) struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Update, move_bullets)
            .add_systems(Update, despawn_bullets)
            .add_systems(Update, bullet_hit_wall.after(despawn_bullets));
    }
}
// pub(crate) fn move_bullets(mut bullets: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
//     for (mut transform, bullet) in &mut bullets {
//         transform.translation += bullet.velocity * time.delta_secs();
//     }
// }
fn despawn_bullets(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Bullet, &Transform), Without<PendingDespawn>>,
    time: Res<Time>,
    game_resources: Res<GameResources>,
) {
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
    bullets: Query<Entity, (With<Bullet>, Without<PendingDespawn>)>,
    mut walls: Query<
        (&mut crate::world::wall::Wall, &mut  crate::world::wall::WallFlash),
        Without<PendingDespawn>,
    >,
) {
    for event in collision_events.read() {
        //here we extract e1 and e2 if the event variant is started. else we continue the loop
        let CollisionEvent::Started(e1, e2, _) = event else {
            continue;
        };
        //extract the entity so we don't have to do duplicate logic, if e1 is in the bullets then its a bullet lol
        //if nighter entity is in bullets then its not a damn bullet, so we continue the loop
        let (bullet_entity, wall_entity) = if bullets.contains(*e1) {
            (*e1, *e2)
        } else if bullets.contains(*e2) {
            (*e2, *e1)
        } else {
            continue;
        };

        //extracting the wall and the flash componeants/entity, if  its a wall (variant ok) else continue
        let Ok((mut wall, mut flash)) = walls.get_mut(wall_entity) else {
            continue;
        };
        //float arithmitic hijenks
        wall.health = (wall.health - 25.0).max(0.0);
        flash.timer = Timer::from_seconds(0.05, TimerMode::Once);
        //que the despawon of the bullet
        commands.entity(bullet_entity).insert(PendingDespawn);
        if wall.health == 0.0 {
            commands.entity(wall_entity).insert(PendingDespawn);
        }
    }
}
