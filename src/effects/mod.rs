use bevy::app::{App, Plugin};
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::math::Rect;
use bevy::prelude::{Commands, Component, Entity, Query, Res, Resource, Sprite, Time, Timer, Update};

pub(crate) struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,animate_smoke);
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
            match smoke_assets.frames.get(smoke.frame_index)  {
                None => {commands.entity(entity).despawn();}
                Some(val) => { sprite.rect=Some(val.clone())}
            }

        }
    }
}