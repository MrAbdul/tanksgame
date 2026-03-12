pub mod player;
pub mod bullet;
pub mod effects;

use bevy::prelude::*;
use bevy::sprite::Anchor;




fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, player::movement::move_sys)
        .add_systems(Update, player::movement::handle_rotations_by_mouse)
        .add_systems(Update, player::shooting::fire_bullet)
        .add_systems(Update, bullet::move_bullets)
        .add_plugins(effects::EffectsPlugin)
        .run()
}

//startup system
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let atlas_handle = asset_server.load("images/tank/sheet_tanks.png");
    let tank_body_rect = Rect::new(671.0, 70.0, 746.0, 140.0);
    let turret_rect = Rect::new(850.0, 58.0, 850.0 + 16.0, 58.0 + 50.0);
    let tank_resources = player::TankResources {
        image: atlas_handle.clone(),
        body: tank_body_rect,
        turret: turret_rect,
    };

    let mut sprite = Sprite::from_image(tank_resources.image.clone());
    sprite.rect = Some(tank_resources.body);
    sprite.flip_y = true;

    let parent = commands
        .spawn((
            player::Player {
                rotation_speed: player::PLAYER_ROTATION_SPEED,
                movement_speed: player::MOVEMENT_SPEED,
            },
            sprite,
            Transform::from_translation(Vec3::new(0f32, 0f32, 1f32)),
        ))
        .id();
    let mut turret_sprite = Sprite::from_image(tank_resources.image.clone());
    turret_sprite.rect = Some(tank_resources.turret);
    let turret = commands
        .spawn((
            player::Turret {
                rotation_speed: player::TURRET_ROTATION_SPEED,
            },
            Anchor::BOTTOM_CENTER,
            turret_sprite,
            Transform::from_translation(Vec3::new(0f32, 0f32, 2f32)),
        ))
        .id();

    commands.entity(parent).add_child(turret);
    commands.insert_resource(tank_resources);
    commands.insert_resource(effects::SmokeAssets {
        image: atlas_handle.clone(),
        frames: vec![
            Rect::new(416.0, 188.0, 416.0 + 87.0, 188.0 + 87.0),
            Rect::new(296.0, 408.0, 296.0 + 92.0, 408.0 + 89.0),
            Rect::new(478.0, 384.0, 478.0 + 90.0, 384.0 + 99.0),
        ],
    })
}






