use bevy::app::{App, Plugin};
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::math::Rect;
use bevy::prelude::{Commands, Component, Entity, IntoScheduleConfigs, Query, Res, Resource, Sprite, Startup, Time, Timer, Update};
use crate::resources;

pub(crate) struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_resources.after(resources::load_resources));
        app.add_systems(Update, animate_smoke);
    }
}

#[derive(Resource)]
pub(crate) struct SmokeAssets {
    pub(crate) image: Handle<Image>,
    pub(crate) frames: Vec<Rect>,
}

#[derive(Component)]
pub(crate) struct SmokeEffect {
    pub(crate) frame_index: usize,
    pub(crate) frame_timer: Timer,
}
pub(crate) fn animate_smoke(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut SmokeEffect)>,
    smoke_assets: Res<SmokeAssets>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut smoke) in &mut query {
        smoke.frame_timer.tick(time.delta());
        if smoke.frame_timer.just_finished() {
            smoke.frame_index += 1;
            match smoke_assets.frames.get(smoke.frame_index) {
                None => {
                    commands.entity(entity).despawn();
                }
                Some(val) => sprite.rect = Some(val.clone()),
            }
        }
    }
}
fn setup_resources(mut commands: Commands, game_resources: Res<resources::GameResources>) {
    commands.insert_resource(SmokeAssets {
        image: game_resources.game_atlas.clone(),
        frames: vec![
            Rect::new(416.0, 188.0, 416.0 + 87.0, 188.0 + 87.0),
            Rect::new(296.0, 408.0, 296.0 + 92.0, 408.0 + 89.0),
            Rect::new(478.0, 384.0, 478.0 + 90.0, 384.0 + 99.0),
        ],
    })
}
