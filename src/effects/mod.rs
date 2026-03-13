use crate::{PendingDespawn};
use bevy::app::{App, Plugin};
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::math::Rect;
use bevy::prelude::{Bundle, Commands, Component, Entity, Query, Res, Sprite, Time, Timer, Transform, Update, Vec3, Without};
use bevy::time::TimerMode;
use crate::resources::GameResources;

pub(crate) struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_smoke);
    }
}

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


pub(crate) fn animate_smoke(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut SmokeEffect),Without<PendingDespawn>>,
    game_resources: Res<GameResources>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut smoke) in &mut query {
        let smoke_assets=&game_resources.effect_resources;
        let sprite_rects = match smoke.smoke_type {
            SmokeType::Grey => &smoke_assets.grey_smoke_frames,
            SmokeType::Yellow => &smoke_assets.yellow_smoke_frames,
        };
        smoke.frame_timer.tick(time.delta());
        if smoke.frame_timer.just_finished() {
            smoke.frame_index += 1;

            match sprite_rects.get(smoke.frame_index) {
                None => {
                    commands.entity(entity).insert(PendingDespawn);
                }
                Some(val) => sprite.rect = Some(val.clone()),
            }
        }
    }
}
