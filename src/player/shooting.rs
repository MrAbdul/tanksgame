use crate::player;
use crate::{bullet};
use bevy::input::ButtonInput;
use bevy::math::{ Vec3};
use bevy::prelude::{
    Commands, GlobalTransform, MouseButton, Res, Single, Time, With
};
use crate::bullet::BulletType;

pub(crate) fn fire_bullet(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Single<(&GlobalTransform, &mut player::Turret), With<player::Turret>>,
    time: Res<Time>,
) {
    let (turret_global, ref mut turret) = *query;
    //ticks the timer
    turret.firing_timer.tick(time.delta());
    //the timer ticker has to be above here to accumulate the ticks

    if !mouse.pressed(MouseButton::Left) {
        return;
    }
    //exit the system if not finished
    if !turret.firing_timer.is_finished() {
        return;
    }

    commands.trigger(bullet::FireEvent{
        base_world_pos:turret_global.transform_point(Vec3::ZERO),
        muzzle_world_pos:turret_global.transform_point(Vec3::new(0.0, 65.0, 0.0)),
        bullet_type:BulletType::Blue,
        global_turret_rotation:turret_global.rotation()
    });

    //reset the timer after firing
    turret.firing_timer.reset()
}
