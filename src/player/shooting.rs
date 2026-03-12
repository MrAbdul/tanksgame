use crate::player;
use crate::{bullet, effects};
use bevy::input::ButtonInput;
use bevy::math::{Rect, Vec3};
use bevy::prelude::{
    Commands, GlobalTransform, MouseButton, Res, Single, Sprite, Time, Timer, TimerMode, Transform,
    Vec3Swizzles, With,
};
use bevy_rapier2d::prelude::*;

pub(crate) fn fire_bullet(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Single<(&GlobalTransform, &mut player::Turret), With<player::Turret>>,
    tank_resources: Res<player::TankResources>,
    smoke_assets: Res<effects::AnimationAssets>,
    time: Res<Time>,
) {
    let (turret_global, ref mut turret) = *query;
    //ticks the timer
    turret.firing_timer.tick(time.delta());
    //the timer ticker has to be above here to accumulate the ticks

    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    //exit the system if not finished
    if !turret.firing_timer.is_finished() {
        return;
    }
    let base_world = turret_global.transform_point(Vec3::ZERO);
    let muzzle_world = turret_global.transform_point(Vec3::new(0.0, 65.0, 0.0));

    let direction = (muzzle_world - base_world).normalize();
    let mut bullet_sprite = Sprite::from_image(tank_resources.image.clone());
    bullet_sprite.rect = Some(Rect::new(148.0, 345.0, 148.0 + 20.0, 345.0 + 33.0));
    let mut transform = Transform::from_translation(muzzle_world);
    transform.rotation = turret_global.rotation();
    commands.spawn((
        bullet_sprite,
        bullet::Bullet {
            lifetime: Timer::from_seconds(1.5, TimerMode::Once),
        },
        transform,
        Collider::cuboid(18.0, 18.0), // half-extents of the sprite rect
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::KINEMATIC_STATIC,

        RigidBody::KinematicVelocityBased,
        Velocity {
            linvel: direction.xy() * 500.0,
            angvel: 0.0,
        },

        Ccd::enabled()
    ));

    commands.spawn(effects::SmokeEffect::new(
        effects::SmokeType::Grey,
        &smoke_assets,
        muzzle_world,
        0.1,
    ));

    //reset the timer after firing
    turret.firing_timer.reset()
}
