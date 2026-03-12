use crate::player;
use bevy::camera::{Camera, Camera2d};
use bevy::input::ButtonInput;
use bevy::math::{EulerRot, Quat, Vec3, Vec3Swizzles};
use bevy::prelude::{
    GlobalTransform, KeyCode, Res, Single, Time, Transform, Window, With, Without,
};
use bevy::window::PrimaryWindow;
use std::f32::consts::FRAC_PI_2;

pub(crate) fn handle_rotations_by_mouse(
    mut turret: Single<
        (&mut Transform, &player::Turret),
        (With<player::Turret>, Without<player::Player>),
    >,
    tank_transform: Single<&Transform, (With<player::Player>, Without<player::Turret>)>,
    camera_query: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    window: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let (camera, camera_transform) = *camera_query;
    let (ref mut transform, turret_cfg) = *turret;

    if let Some(position) = window.cursor_position()
        && let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, position)
    {
        let tank_pos = tank_transform.translation.xy();
        let to_cursor = cursor_world_pos - tank_pos;

        if to_cursor.length_squared() == 0.0 {
            return;
        }

        let desired_world_angle = to_cursor.to_angle() - FRAC_PI_2;
        let tank_body_angle = tank_transform.rotation.to_euler(EulerRot::XYZ).2;
        let desired_local_angle = desired_world_angle - tank_body_angle;

        let target_rotation = Quat::from_rotation_z(desired_local_angle);
        let max_step = turret_cfg.rotation_speed * time.delta_secs();

        transform.rotation = transform.rotation.rotate_towards(target_rotation, max_step);
    }
}
pub(crate) fn move_sys(
    mut player: Single<(&mut Transform, &player::Player), With<player::Player>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (ref mut transform, player) = *player;

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;
    if input.pressed(KeyCode::ArrowLeft) {
        rotation_factor += 1.0;
    }

    if input.pressed(KeyCode::ArrowRight) {
        rotation_factor -= 1.0;
    }

    if input.pressed(KeyCode::ArrowUp) {
        movement_factor += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        movement_factor -= 1.0;
    }
    transform.rotate_z(rotation_factor * player.rotation_speed * time.delta_secs());
    // Get the ship's forward vector by applying the current rotation to the ships initial facing
    // vector
    let movement_direction = transform.rotation * Vec3::Y;
    let movement_distance = movement_factor * player.movement_speed * time.delta_secs();
    // Create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // Update the ship translation with our new translation delta
    transform.translation += translation_delta;
}
