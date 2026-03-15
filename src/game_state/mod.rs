use bevy::prelude::*;
use crate::{enemy, player};

#[derive(States,Default,Clone,PartialEq,Eq,Hash,Debug)]
pub(crate) enum GameState {
    #[default]
    Loading,
    MainMenu,
    Playing,
    Won,
    Lost,
}
pub(crate) struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, check_win.run_if(in_state(GameState::Playing)))
            .add_systems(Update, check_loss.run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Playing), reset_game);

    }
}
fn check_win(
    enemies: Query<(), With<enemy::Enemy>>,
    mut next_state: ResMut<NextState<GameState>>,
    // only trigger after map has spawned
    map_spawned: Option<Res<crate::world::MapSpawned>>,
) {
    let Some(_) = map_spawned else { return; };
    if enemies.is_empty() {
        next_state.set(GameState::Won);
    }
}
pub(crate) fn reset_game(
    mut commands: Commands,
    entities: Query<Entity, Or<(
        With<crate::player::Player>,
        With<crate::enemy::Enemy>,
        With<crate::bullet::Bullet>,
        With<crate::world::wall::Wall>,
    )>>,
    map_spawned: Option<Res<crate::world::MapSpawned>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
    if map_spawned.is_some() {
        commands.remove_resource::<crate::world::MapSpawned>();
    }
}
fn check_loss(
    player: Query<(), With<player::Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    map_spawned: Option<Res<crate::world::MapSpawned>>,
) {
    let Some(_) = map_spawned else { return; };
    if player.is_empty() {
        next_state.set(GameState::Lost);
    }
}