use crate::bullet;
use crate::bullet::BulletType;
use crate::enemy::{Enemy, EnemyTurret};
use crate::player::Player;
use crate::resources::GameConfig;
use bevy::math::{EulerRot, Quat, Vec2, Vec3, Vec3Swizzles};
use bevy::prelude::{
    Children, Commands, Entity, GlobalTransform, Query, Res, Single, Transform, With, Without,
};
use bevy::time::Time;
use bevy_rapier2d::prelude::ReadRapierContext;
use bevy_rapier2d::prelude::{QueryFilter, Real, Velocity};
use std::f32::consts::FRAC_PI_2;
pub(crate) fn rotate_turret_towards_player(
    player: Single<&Transform, With<Player>>,
    mut enemy_turret_query: Query<
        (&mut Transform, &mut EnemyTurret, &GlobalTransform),
        (With<EnemyTurret>, Without<Enemy>, Without<Player>),
    >,
    tank: Query<(&Transform, &Children), (With<Enemy>, Without<EnemyTurret>, Without<Player>)>,
    time: Res<Time>,
    game_config: Option<Res<GameConfig>>,
    mut commands: Commands,
) {
    let Some(game_config) = game_config else {
        return;
    };

    let player_pos = player.translation.xy();

    for (transform, child_comp) in tank.iter() {
        for child in child_comp.iter() {
            if let Ok(child) = enemy_turret_query.get_mut(*child) {
                let (mut transform_child, mut turret, global_transform) = child;
                let to_player = player_pos - transform.translation.xy();
                // added distance to know when to fire
                let distance = player_pos.distance(transform.translation.xy());

                if distance < 50.0 || distance > 1000.0 {
                    continue;
                }
                let desired_world_angle = to_player.to_angle() - FRAC_PI_2;
                let tank_body_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
                let desired_local_angle = desired_world_angle - tank_body_angle;

                let target_rotation = Quat::from_rotation_z(desired_local_angle);
                let angle_def = target_rotation.angle_between(transform_child.rotation);
                //to know when to fire
                if angle_def <= game_config.enemy_targeting_angle.to_radians() {
                    //when the enemy is pointed towards the player ticke the timer
                    turret.firing_timer.tick(time.delta());
                    if turret.firing_timer.is_finished() {
                        commands.trigger(bullet::FireEvent {
                            base_world_pos: global_transform.transform_point(Vec3::ZERO),
                            muzzle_world_pos: global_transform
                                .transform_point(Vec3::new(0.0, 65.0, 0.0)),
                            bullet_type: BulletType::Red,
                            global_turret_rotation: global_transform.rotation(),
                        });
                        turret.firing_timer.reset()
                    }
                }
                let max_step = turret.rotation_speed * time.delta_secs();
                transform_child.rotation = transform_child
                    .rotation
                    .rotate_towards(target_rotation, max_step);
            }
        }
    }
}

pub(crate) fn move_towards_player(
    player: Single<&Transform, With<Player>>,
    mut enemies: Query<
        (Entity, &mut Transform, &mut Enemy, &mut Velocity),
        (With<Enemy>, Without<Player>),
    >,
    rapier_context: ReadRapierContext,
    time: Res<Time>,
    game_config: Option<Res<GameConfig>>,
) {
    let Ok(rapier_context) = rapier_context.single() else { return; };
    let Some(game_config) = game_config else { return; };
    let player_pos = player.translation.xy();

    for (entity,  mut transform, mut enemy, mut velocity) in &mut enemies {
        let enemy_pos = transform.translation.xy();
        let to_player = player_pos - enemy_pos;
        let distance = to_player.length();
        if distance <= game_config.enemy_stop_distance {
            velocity.linvel = Vec2::ZERO;
            enemy.stuck_timer = 0.0;

            continue;
        }

        let desired_dir = to_player.normalize_or_zero();
        let max_toi:Real = game_config.enemy_raycast_distance;
        let filter = QueryFilter::default().exclude_rigid_body(entity);

        // single forward ray
        let mut steer_dir = desired_dir;

        if let Some(_) = rapier_context.cast_ray(enemy_pos, desired_dir, max_toi, true, filter) {
            // obstacle ahead — check right and left
            let right = Vec2::new(desired_dir.y, -desired_dir.x);
            let left  = Vec2::new(-desired_dir.y,  desired_dir.x);

            let right_clear = rapier_context
                .cast_ray(enemy_pos, right, max_toi, true, filter)
                .is_none();

            steer_dir = if right_clear { right } else { left };
        }

        // stuck detection
        println!("{}",enemy_pos.distance(player_pos));

        if velocity.linvel.length() < 20.0 ||!(enemy_pos.distance(player_pos) < (enemy.previous_distance_to_player - enemy_pos.distance(player_pos)) )  {

            enemy.stuck_timer += time.delta_secs();
            println!("stuck detecting adding to stuck timer{}",enemy.stuck_timer);

        } else {
            enemy.stuck_timer = 0.0;

        }
        if enemy.stuck_timer > 0.5 {
            println!("stuck detecting flipping direction {}",enemy.stuck_timer);
            steer_dir = Vec2::new(-desired_dir.y, desired_dir.x);
            enemy.stuck_timer = 0.0;
        }


        // rotate body toward steer direction
        let desired_angle = steer_dir.to_angle() - FRAC_PI_2;
        let current_angle = transform.rotation.to_euler(EulerRot::XYZ).2;

        let mut angle_diff = desired_angle - current_angle;
        while angle_diff >  std::f32::consts::PI  { angle_diff -= std::f32::consts::TAU; }
        while angle_diff < -std::f32::consts::PI  { angle_diff += std::f32::consts::TAU; }

        let max_rot = game_config.enemy_body_rotation_speed * time.delta_secs();
        transform.rotate_z(angle_diff.clamp(-max_rot, max_rot));
        enemy.previous_distance_to_player=enemy_pos.distance(player_pos);

        // move in direction body is facing
        let forward = (transform.rotation * Vec3::Y).xy();
        velocity.linvel = forward * enemy.movement_speed;
    }
}
