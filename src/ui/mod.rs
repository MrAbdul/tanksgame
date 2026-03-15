use bevy::prelude::*;
use crate::game_state::GameState;
use crate::enemy::Enemy;
use crate::player::Turret;

pub struct UiPlugin;

// marker components so we can despawn screens cleanly
#[derive(Component)] struct MainMenuScreen;
#[derive(Component)] struct WinScreen;
#[derive(Component)] struct LoseScreen;
#[derive(Component)] struct HudScreen;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // spawn screens on state enter
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnEnter(GameState::Playing), spawn_hud)
            .add_systems(OnEnter(GameState::Won), spawn_win_screen)
            .add_systems(OnEnter(GameState::Lost), spawn_lose_screen)
            // despawn screens on state exit
            .add_systems(OnExit(GameState::MainMenu), despawn_screen::<MainMenuScreen>)
            .add_systems(OnExit(GameState::Playing), despawn_screen::<HudScreen>)
            .add_systems(OnExit(GameState::Won), despawn_screen::<WinScreen>)
            .add_systems(OnExit(GameState::Lost), despawn_screen::<LoseScreen>)
            // button interactions
            .add_systems(Update, handle_play_button
                .run_if(in_state(GameState::MainMenu)))
            .add_systems(Update, handle_restart_button
                .run_if(in_state(GameState::Won).or(in_state(GameState::Lost))))
            // hud update only while playing
            .add_systems(Update, update_hud
                .run_if(in_state(GameState::Playing)));
    }
}

// --- MAIN MENU ---

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        MainMenuScreen,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
    )).with_children(|parent| {
        // title
        parent.spawn((
            Text::new("TANK ON EMPTY"),
            TextFont { font_size: 64.0, ..default() },
            TextColor::WHITE,
        ));

        // play button
        parent.spawn((
            Button,
            Node {
                padding: UiRect::axes(Val::Px(40.0), Val::Px(15.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
        )).with_children(|btn| {
            btn.spawn((
                Text::new("PLAY"),
                TextFont { font_size: 32.0, ..default() },
                TextColor::WHITE,
            ));
        });
    });
}

fn handle_play_button(
    interaction: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

// --- WIN SCREEN ---

fn spawn_win_screen(mut commands: Commands) {
    spawn_end_screen(&mut commands, "YOU WIN!", Color::srgb(0.2, 0.8, 0.2), WinScreen);
}

// --- LOSE SCREEN ---

fn spawn_lose_screen(mut commands: Commands) {
    spawn_end_screen(&mut commands, "GAME OVER", Color::srgb(0.8, 0.2, 0.2), LoseScreen);
}

fn spawn_end_screen<M: Component>(
    commands: &mut Commands,
    title: &str,
    color: Color,
    marker: M,
) {
    commands.spawn((
        marker,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            Text::new(title),
            TextFont { font_size: 64.0, ..default() },
            TextColor(color),
        ));

        // restart button
        parent.spawn((
            Button,
            Node {
                padding: UiRect::axes(Val::Px(40.0), Val::Px(15.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.4, 0.8)),
        )).with_children(|btn| {
            btn.spawn((
                Text::new("RESTART"),
                TextFont { font_size: 32.0, ..default() },
                TextColor::WHITE,
            ));
        });
    });
}

fn handle_restart_button(
    interaction: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

// --- HUD ---

#[derive(Component)] struct EnemyCountText;
#[derive(Component)] struct CoolDownText;

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        HudScreen,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Enemies: 0"),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(15.0),
                left: Val::Px(15.0),
                ..default()
            },
            TextColor::WHITE,
            EnemyCountText,
        ));

        parent.spawn((
            Text::new("Cooldown: READY"),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(40.0),
                left: Val::Px(15.0),
                ..default()
            },
            TextColor::WHITE,
            CoolDownText,
        ));
    });
}

fn update_hud(
    enemy_query: Query<(), With<Enemy>>,
    turret_query: Query<&Turret, With<Turret>>,
    mut enemy_text: Query<&mut Text, (With<EnemyCountText>, Without<CoolDownText>)>,
    mut cooldown_text: Query<(&mut Text, &mut TextColor), (With<CoolDownText>, Without<EnemyCountText>)>,
) {
    if let Ok(mut text) = enemy_text.single_mut() {
        **text = format!("Enemies: {}", enemy_query.iter().count());
    }

    if let Ok(turret) = turret_query.single() {
        if let Ok((mut text, mut color)) = cooldown_text.single_mut() {
            let remaining = turret.firing_timer.remaining_secs();
            if remaining <= 0.0 {
                **text = "Cooldown: READY".into();
                color.0 = Color::srgb(0.2, 0.8, 0.2);
            } else {
                **text = format!("Cooldown: {:.2}", remaining);
                color.0 = Color::srgb(0.8, 0.2, 0.2);
            }
        }
    }
}

// --- UTILITY ---

fn despawn_screen<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}