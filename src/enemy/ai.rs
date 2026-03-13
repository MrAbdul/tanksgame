use bevy::math::{EulerRot, Quat, Vec3, Vec3Swizzles};
use bevy::prelude::{Children, Commands, GlobalTransform, Query, Res, Single, Transform, With, Without};
use bevy::time::Time;
use std::f32::consts::FRAC_PI_2;
use crate::bullet;
use crate::bullet::BulletType;
use crate::enemy::{Enemy, EnemyTurret};
use crate::player::Player;

pub(crate) fn rotate_turret_towards_player(
    player: Single<&Transform, With<Player>>,
    mut enemy_turret_query: Query<
        (&mut Transform, &mut EnemyTurret,&GlobalTransform),
        (With<EnemyTurret>, Without<Enemy>, Without<Player>),
    >,
    tank: Query<(&Transform, &Children), (With<Enemy>, Without<EnemyTurret>)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let player_pos = player.translation.xy();

    for (transform, child_comp) in tank.iter() {
        for child in child_comp.iter() {
            if let Ok(child) = enemy_turret_query.get_mut(*child) {
                let (mut transform_child, mut turret,global_transform) = child;
                let to_player = player_pos - transform.translation.xy();
                if to_player.length_squared() == 0.0 {
                    return;
                }
                let desired_world_angle = to_player.to_angle() - FRAC_PI_2;
                let tank_body_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
                let desired_local_angle = desired_world_angle - tank_body_angle;

                let target_rotation = Quat::from_rotation_z(desired_local_angle);
                let angle_def=target_rotation.angle_between(transform_child.rotation);
                //to know when to fire
                if angle_def<=5.0f32.to_radians(){
                    //when the enemy is pointed towards the player ticke the timer
                    turret.firing_timer.tick(time.delta());
                    if turret.firing_timer.is_finished() {


                    commands.trigger(bullet::FireEvent{
                        base_world_pos:global_transform.transform_point(Vec3::ZERO),
                        muzzle_world_pos:global_transform.transform_point(Vec3::new(0.0, 65.0, 0.0)),
                        bullet_type:BulletType::Red,
                        global_turret_rotation:global_transform.rotation()
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
