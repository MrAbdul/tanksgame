use bevy::input::ButtonInput;
use bevy::math::{Rect, Vec3};
use bevy::prelude::{Commands, GlobalTransform, MouseButton, Res, Single, Sprite, Timer, TimerMode, Transform, With};
use crate::{bullet, effects,};
use crate::player;

pub(crate) fn fire_bullet(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    turret: Single<&GlobalTransform, With<player::Turret>>,
    tank_resources: Res<player::TankResources>,
    smoke_assets: Res<effects::SmokeAssets>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let turret_global = *turret;

    let base_world = turret_global.transform_point(Vec3::ZERO);
    let muzzle_world = turret_global.transform_point(Vec3::new(0.0, 65.0, 0.0));

    let direction = (muzzle_world - base_world).normalize();
    let mut bullet_sprite = Sprite::from_image(tank_resources.image.clone());
    bullet_sprite.rect = Some(Rect::new(148.0, 345.0, 148.0 + 20.0, 345.0 + 33.0));
    let mut transform = Transform::from_translation(muzzle_world);
    transform.rotation = turret.rotation();
    commands.spawn((
        bullet_sprite,
        bullet::Bullet {
            velocity: direction * 500.0,
        },
        transform,
    ));
    let mut sprite = Sprite::from_image(smoke_assets.image.clone());
    sprite.rect = smoke_assets.frames.get(0).cloned();
    // sprite.custom_size=Some(Vec2::new(16.0, 16.0));
    commands.spawn((
        Transform::from_translation(muzzle_world),
        sprite,
        effects::SmokeEffect {
            frame_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frame_index: 0,
        },
    ));
}
