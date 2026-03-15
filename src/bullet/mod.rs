use crate::player::Player;
use crate::resources::GameResources;
use crate::{audio, bullet, effects, enemy, health, player, resources, world, PendingDespawn};
use bevy::prelude::*;
use bevy::time::Timer;
use bevy_rapier2d::prelude::*;
use crate::game_state::GameState;

#[derive(Component)]
pub(crate) struct Bullet {
    pub(crate) lifetime: Timer,
    bullet_type: BulletType,
}

#[derive(Event)]
pub(crate) struct FireEvent {
    pub(crate) muzzle_world_pos: Vec3,
    pub(crate) base_world_pos: Vec3,
    pub(crate) bullet_type: BulletType,
    pub(crate) global_turret_rotation: Quat,
}

#[derive(Debug, Clone, PartialEq, Eq)]

pub(crate) enum BulletType {
    Blue,
    Red,
}
pub(crate) struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Update, move_bullets)
            .add_systems(Update, despawn_bullets.run_if(in_state(GameState::Playing)))
            .add_observer(on_fire)
            .add_systems(Update, proccess_bullet_collisions.after(despawn_bullets).run_if(in_state(GameState::Playing)));
    }
}
pub(crate) enum EntityType {
    Player(Entity),
    Enemy(Entity),
    Wall(Entity),
    Bullet(Entity),
}
//an event observer
fn on_fire(
    fire: On<FireEvent>,
    game_resources: Res<GameResources>,
    mut commands: Commands,
    game_config: Option<Res<resources::GameConfig>>,
    audio_resource: Option<Res<audio::GameAudio>>,
    player_transform: Query<&Transform, With<Player>>,
) {
    let Some(game_config) = game_config else {
        return;
    };
    let Some(audio_resource) = audio_resource else {
        return;
    };
    let Ok(player_transform) = player_transform.single() else {
        return;
    };

    let mut sprite = Sprite::from_image(game_resources.game_atlas.clone());
    sprite.rect = Some(match fire.bullet_type {
        BulletType::Blue => game_resources.bullet_atlas_rect,
        BulletType::Red => game_resources.bullet_enemy_atlas_rect,
    });
    let direction = (fire.muzzle_world_pos - fire.base_world_pos).normalize();
    let mut transform = Transform::from_translation(fire.muzzle_world_pos);
    transform.rotation = fire.global_turret_rotation;
    commands.spawn((
        sprite,
        bullet::Bullet {
            lifetime: Timer::from_seconds(1.5, TimerMode::Once),
            bullet_type: match fire.bullet_type {
                BulletType::Blue => BulletType::Blue,
                BulletType::Red => BulletType::Red,
            },
        },
        transform,
        Collider::cuboid(18.0, 18.0), // half-extents of the sprite rect
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::KINEMATIC_STATIC,
        RigidBody::KinematicVelocityBased,
        Velocity {
            linvel: direction.xy()
                * match fire.bullet_type {
                    BulletType::Blue => game_config.player_bullet_base_velocity,
                    BulletType::Red => game_config.enemy_bullet_base_velocity,
                },
            angvel: 0.0,
        },
        Ccd::enabled(),
    ));

    audio::play_one_shot(
        &mut commands,
        audio_resource.player_fire.clone(),
        match fire.bullet_type {
            BulletType::Blue => 0.7,
            BulletType::Red => {
                let distance = fire.muzzle_world_pos.distance(player_transform.translation);
                //modulate the value
                let mapped = 1.0 - (((distance - 50.0) / (1000.0 - 50.0)).clamp(0.0, 1.0));
                mapped
            }
        },
    );

    commands.spawn(effects::SmokeEffect::new(
        effects::SmokeType::Grey,
        &game_resources.effect_resources,
        fire.muzzle_world_pos,
        game_config.muzzle_smoke_effect_frame_duration,
    ));
}
fn despawn_bullets(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Bullet, &Transform), Without<PendingDespawn>>,
    time: Res<Time>,
    game_config: Option<Res<resources::GameConfig>>,
    game_resources: Res<GameResources>,
    audio_resource: Option<Res<audio::GameAudio>>,
) {
    let Some(game_config) = game_config else {
        return;
    };
    let Some(audio_resource) = audio_resource else {
        return;
    };

    for (entity, mut bullet, transform) in &mut bullets {
        bullet.lifetime.tick(time.delta());
        if bullet.lifetime.is_finished() {
            audio::play_one_shot(&mut commands, audio_resource.explosion.clone(), 0.1);

            commands.spawn(effects::SmokeEffect::new(
                effects::SmokeType::Yellow,
                &game_resources.effect_resources,
                transform.translation,
                game_config.bullet_explosion_effect_frame_duration,
            ));
            commands.entity(entity).insert(PendingDespawn);
        }
    }
}
pub(crate) fn proccess_bullet_collisions(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
    bullets: Query<(Entity, &Bullet), (With<Bullet>, Without<PendingDespawn>)>,
    mut enemies: Query<
        (&mut health::Health, &health::TakesDamageFrom),
        (Without<PendingDespawn>,
         With<enemy::Enemy>,
         Without<player::Player>,
         Without<world::wall::Wall>,),
    >,
    mut player: Query<
        (&mut health::Health, &health::TakesDamageFrom),
        (Without<PendingDespawn>,
         With<player::Player>,
         Without<enemy::Enemy>,
         Without<world::wall::Wall>,),
    >,
    mut walls: Query<
        (
            &mut health::Health,
            &health::TakesDamageFrom,
            &mut world::wall::WallFlash,
        ),
        (Without<PendingDespawn>,
         With<world::wall::Wall>,
         Without<enemy::Enemy>,
         Without<player::Player>,),
    >,

    game_config: Option<Res<resources::GameConfig>>,
    audio_resource: Option<Res<audio::GameAudio>>,
) {
    let Some(game_config) = game_config else {
        return;
    };
    let Some(audio_resource) = audio_resource else {
        return;
    };
    for event in collision_events.read() {
        //here we extract e1 and e2 if the event variant is started. else we continue the loop
        let CollisionEvent::Started(e1, e2, _) = event else {
            continue;
        };

        //extract the entity so we don't have to do duplicate logic, if e1 is in the bullets then its a bullet lol
        //if nighter entity is in bullets then its not a damn bullet, so we continue the loop

        let (bullet, hittee_entity_type) = if bullets.contains(*e1) && walls.contains(*e2) {
            (*e1, EntityType::Wall(*e2))
        } else if bullets.contains(*e2) && walls.contains(*e1) {
            (*e2, EntityType::Wall(*e1))
        }
        else if bullets.contains(*e1)&&enemies.contains(*e2) {
            (*e1,  EntityType::Enemy(*e2))
        }
        else if bullets.contains(*e2)&&enemies.contains(*e1) {
            (*e2,  EntityType::Enemy(*e1))
        }
        else if bullets.contains(*e1)&&player.contains(*e2) {
            (*e1,  EntityType::Player(*e2))
        }
        else if bullets.contains(*e2)&&player.contains(*e1) {
            (*e2,  EntityType::Player(*e1))
        }
        else {
            continue;
        };
        let Ok(bullet) = bullets.get(bullet) else {
            continue;
        };
        let (bullet_entity, bullet) = bullet;
        match hittee_entity_type {
            EntityType::Player(e) => {
                let Ok((mut health, takes_damage_from)) = player.get_mut(e) else {
                    continue;
                };

                let takes_damage: bool = takes_damage_from
                    .damaging_bullets
                    .contains(&bullet.bullet_type);
                if takes_damage {
                    audio::play_one_shot(&mut commands, audio_resource.wall_hit.clone(), 0.8);
                    health.health = (health.health - 25.0).max(0.0);
                    if health.health == 0.0 {
                        commands.entity(e).insert(PendingDespawn);
                    }
                    commands.entity(bullet_entity).insert(PendingDespawn);
                }
            }
            EntityType::Enemy(e) => {
                let Ok((mut health, takes_damage_from)) = enemies.get_mut(e) else {
                    continue;
                };

                let takes_damage: bool = takes_damage_from
                    .damaging_bullets
                    .contains(&bullet.bullet_type);
                if takes_damage {
                    audio::play_one_shot(&mut commands, audio_resource.wall_hit.clone(), 0.8);
                    health.health = (health.health - 25.0).max(0.0);
                    if health.health == 0.0 {
                        commands.entity(e).insert(PendingDespawn);
                    }
                    commands.entity(bullet_entity).insert(PendingDespawn);
                }
            }
            EntityType::Wall(e) => {
                let Ok((mut health, takes_damage_from, mut wall_flash)) = walls.get_mut(e) else {
                    continue;
                };

                let takes_damage: bool = takes_damage_from
                    .damaging_bullets
                    .contains(&bullet.bullet_type);
                if takes_damage {
                    audio::play_one_shot(&mut commands, audio_resource.wall_hit.clone(), 0.8);
                    health.health =
                        (health.health - game_config.bullet_wall_damage_amount).max(0.0);
                    wall_flash.timer =
                        Timer::from_seconds(game_config.wall_hit_flash_duration, TimerMode::Once);
                    if health.health == 0.0 {
                        commands.entity(e).insert(PendingDespawn);
                    }
                    commands.entity(bullet_entity).insert(PendingDespawn);
                }
            }
            _ => (),
        }

        // //extracting the wall and the flash componeants/entity, if  its a wall (variant ok) else continue
        // let Ok((mut wall, mut flash)) = walls.get_mut(wall_entity) else {
        //     continue;
        // };
        // //float arithmitic hijenks
        // audio::play_one_shot(&mut commands, audio_resource.wall_hit.clone(), 0.8);
        // wall.health = (wall.health - game_config.bullet_wall_damage_amount).max(0.0);
        // flash.timer = Timer::from_seconds(game_config.wall_hit_flash_duration, TimerMode::Once);
        // //que the despawon of the bullet
        // commands.entity(bullet_entity).insert(PendingDespawn);
        // if wall.health == 0.0 {
        //     commands.entity(wall_entity).insert(PendingDespawn);
        // }
    }
}
