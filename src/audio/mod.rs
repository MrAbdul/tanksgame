use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings, Volume};
use bevy::prelude::*;

pub(crate) struct GameAudioPlugin;

#[derive(Resource)]
pub(crate) struct GameAudio {
    pub(crate) player_fire: Handle<AudioSource>,
    pub(crate) enemy_fire: Handle<AudioSource>,
    pub(crate) wall_hit: Handle<AudioSource>,
    pub(crate) explosion: Handle<AudioSource>,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_audio);
    }
}

fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAudio {
        player_fire: asset_server.load("sound_effects/tank-firing.ogg"),
        enemy_fire: asset_server.load("sound_effects/tank-firing.ogg"),
        wall_hit: asset_server.load("sound_effects/bullet_hit_wall.ogg"),
        explosion: asset_server.load("sound_effects/bullet_exploads.ogg"),
    });
}

pub(crate) fn play_one_shot(commands: &mut Commands, sound: Handle<AudioSource>, volume: f32) {
    commands.spawn((
        AudioPlayer::new(sound),
        PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume)),
    ));
}