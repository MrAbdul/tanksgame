use bevy::prelude::KeyCode;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use std::f32::consts::FRAC_PI_2;

const MOVEMENT_SPEED: f32 = 300.0;
const PLAYER_ROTATION_SPEED: f32 = 2.1;
const TURRET_ROTATION_SPEED: f32 = 3.1;
#[derive(Component)]
struct Player {
    movement_speed: f32,
    rotation_speed: f32,
}
#[derive(Component)]
struct Turret {
    rotation_speed: f32,
}

#[derive(Component)]
struct Bullet {
    velocity: Vec3,
}
#[derive(Resource)]
struct SmokeAssets {
    image: Handle<Image>,
    frames: Vec<Rect>,
}
#[derive(Resource)]
struct TankResources {
    image: Handle<Image>,
    body: Rect,
    turret: Rect,
}
#[derive(Component)]
struct SmokeEffect {
    frame_index: usize,
    frame_timer: Timer,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, move_sys)
        .add_systems(Update, handle_rotations_by_mouse)
        .add_systems(Update, fire_bullet)
        .add_systems(Update, move_bullets)
        .add_systems(Update, animate_smoke)
        .run()
}

//startup system
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let atlas_handle = asset_server.load("images/tank/sheet_tanks.png");
    let tank_body_rect = Rect::new(671.0, 70.0, 746.0, 140.0);
    let turret_rect = Rect::new(850.0, 58.0, 850.0 + 16.0, 58.0 + 50.0);
    let tank_resources = TankResources {
        image: atlas_handle.clone(),
        body: tank_body_rect,
        turret: turret_rect,
    };

    let mut sprite = Sprite::from_image(tank_resources.image.clone());
    sprite.rect = Some(tank_resources.body);
    sprite.flip_y = true;

    let parent = commands
        .spawn((
            Player {
                rotation_speed: PLAYER_ROTATION_SPEED,
                movement_speed: MOVEMENT_SPEED,
            },
            sprite,
            Transform::from_translation(Vec3::new(0f32, 0f32, 1f32)),
        ))
        .id();
    let mut turret_sprite = Sprite::from_image(tank_resources.image.clone());
    turret_sprite.rect = Some(tank_resources.turret);
    let turret = commands
        .spawn((
            Turret {
                rotation_speed: TURRET_ROTATION_SPEED,
            },
            Anchor::BOTTOM_CENTER,
            turret_sprite,
            Transform::from_translation(Vec3::new(0f32, 0f32, 2f32)),
        ))
        .id();

    commands.entity(parent).add_child(turret);
    commands.insert_resource(tank_resources);
    commands.insert_resource(SmokeAssets {
        image: atlas_handle.clone(),
        frames: vec![
            Rect::new(416.0, 188.0, 416.0 + 87.0, 188.0 + 87.0),
            Rect::new(296.0, 408.0, 296.0 + 92.0, 408.0 + 89.0),
            Rect::new(478.0, 384.0, 478.0 + 90.0, 384.0 + 99.0),
        ],
    })
}

fn handle_rotations_by_mouse(
    mut turret: Single<(&mut Transform, &Turret), (With<Turret>, Without<Player>)>,
    tank_transform: Single<&Transform, (With<Player>, Without<Turret>)>,
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
fn move_sys(
    mut player: Single<(&mut Transform, &Player), With<Player>>,
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

fn fire_bullet(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    turret: Single<&GlobalTransform, With<Turret>>,
    tank_resources: Res<TankResources>,
    smoke_assets: Res<SmokeAssets>,
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
        Bullet {
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
        SmokeEffect {
            frame_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frame_index: 0,
        },
    ));
}
fn animate_smoke(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut SmokeEffect)>,
    smoke_assets: Res<SmokeAssets>,
    time: Res<Time>,
) {

    for (entity, mut sprite, mut smoke) in &mut query {
        smoke.frame_timer.tick(time.delta());
        if smoke.frame_timer.just_finished() {
            smoke.frame_index += 1;
            match smoke_assets.frames.get(smoke.frame_index)  {
                None => {commands.entity(entity).despawn();}
                Some(val) => { sprite.rect=Some(val.clone())}
            }

        }
    }
}
fn move_bullets(mut bullets: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in &mut bullets {
        transform.translation += bullet.velocity * time.delta_secs();
    }
}
