use crate::resources;
use bevy::app::{App, Plugin};
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::math::Rect;
use bevy::prelude::{
    Bundle, Commands, Component, Entity, IntoScheduleConfigs, Query, Res, Resource, Sprite,
    Startup, Time, Timer, Transform, Update, Vec3,
};
use bevy::time::TimerMode;

pub(crate) struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_resources.after(resources::load_resources));
        app.add_systems(Update, animate_smoke);
    }
}

#[derive(Resource)]
pub(crate) struct AnimationAssets {
    pub(crate) image: Handle<Image>,
    pub(crate) grey_smoke_frames: Vec<Rect>,
    pub(crate) yellow_smoke_frames: Vec<Rect>,
}

pub(crate) enum SmokeType {
    Grey,
    Yellow,
}
#[derive(Component)]
pub(crate) struct SmokeEffect {
    pub(crate) frame_index: usize,
    pub(crate) frame_timer: Timer,
    pub(crate) smoke_type: SmokeType,
}
impl SmokeEffect {
    pub(crate) fn new(
        smoke_type: SmokeType,
        assets: &AnimationAssets,
        position: Vec3,
        time: f32,
    ) -> impl Bundle {
        let frames = match smoke_type {
            SmokeType::Grey => &assets.grey_smoke_frames,
            SmokeType::Yellow => &assets.yellow_smoke_frames,
        };
        let mut sprite = Sprite::from_image(assets.image.clone());
        sprite.rect = frames.get(0).cloned();

        (
            Transform::from_translation(position),
            sprite,
            SmokeEffect {
                frame_timer: Timer::from_seconds(time, TimerMode::Repeating),
                frame_index: 0,
                smoke_type,
            },
        )
    }
}
fn setup_resources(mut commands: Commands, game_resources: Res<resources::GameResources>) {
    commands.insert_resource(AnimationAssets {
        image: game_resources.game_atlas.clone(),
        grey_smoke_frames: vec![
            Rect::new(416.0, 188.0, 416.0 + 87.0, 188.0 + 87.0),
            Rect::new(296.0, 408.0, 296.0 + 92.0, 408.0 + 89.0),
            Rect::new(478.0, 384.0, 478.0 + 90.0, 384.0 + 99.0),
        ],
        yellow_smoke_frames: vec![
            Rect::new(228.0, 107.0, 228.0 + 87.0, 107.0 + 87.0),
            Rect::new(326.0, 0.0, 326.0 + 92.0, 0.0 + 89.0),
            Rect::new(416.0, 89.0, 416.0 + 90.0, 89.0 + 99.0),
            Rect::new(651.0, 432.0, 651.0 + 79.0, 432.0 + 79.0),
            Rect::new(100.0, 384.0, 100.0 + 100.0, 384.0 + 97.0),
            Rect::new(298.0, 301.0, 298.0 + 98.0, 301.0 + 107.0),
        ],
    })
}

pub(crate) fn animate_smoke(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut SmokeEffect)>,
    smoke_assets: Res<AnimationAssets>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut smoke) in &mut query {
        let sprite_rects = match smoke.smoke_type {
            SmokeType::Grey => &smoke_assets.grey_smoke_frames,
            SmokeType::Yellow => &smoke_assets.yellow_smoke_frames,
        };
        smoke.frame_timer.tick(time.delta());
        if smoke.frame_timer.just_finished() {
            smoke.frame_index += 1;

            match sprite_rects.get(smoke.frame_index) {
                None => {
                    commands.entity(entity).despawn();
                }
                Some(val) => sprite.rect = Some(val.clone()),
            }
        }
    }
}
