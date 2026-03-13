
use bevy::app::{App, Startup};
use bevy::color::palettes::css::{GREEN, RED};
use bevy::prelude::*;
use bevy::ui::{px, PositionType};
use bevy::utils::default;
use crate::{enemy, player};

pub struct UiPlugin;
#[derive(Component)]
struct EnemyCountText;

#[derive(Component)]
struct CoolDownText;

impl Plugin for UiPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_hud)
            .add_systems(Update, update_hud);
    }
}

fn spawn_hud(mut commands: Commands){
    commands.spawn((
        Text::new("Enemies:0"),
        Node{
            position_type:PositionType::Absolute,
            top:px(15.0),
            left:px(15.0),
            ..default()
        },
        TextColor::WHITE,
        EnemyCountText,
        ));

    commands.spawn((
        Text::new("Cooldown: READY"),
        Node {
            position_type: PositionType::Absolute,
            top: px(40.0),
            left: px(15.0),
            ..default()
        },
        TextColor::WHITE,
        CoolDownText,
    ));
}
fn update_hud(
    enemy_query: Query<(), With<enemy::Enemy>>,
    turret_query: Query<&player::Turret, With<player::Turret>>,
    mut enemy_text_query: Query<(&mut Text, &mut TextColor), (With<EnemyCountText>, Without<CoolDownText>)>,
    mut cooldown_text_query: Query<(&mut Text, &mut TextColor), (With<CoolDownText>, Without<EnemyCountText>)>,
) {
    if let Ok( text_opt) = enemy_text_query.single_mut() {
        let (mut text,_)=text_opt;
        **text = format!("Enemies: {}", enemy_query.iter().count());
    }

    if let Ok(turret) = turret_query.single() {
        if let Ok( text_opts) = cooldown_text_query.single_mut() {
            let (mut text,mut text_color)=text_opts;

            let remaining = turret.firing_timer.remaining_secs();
            if remaining <= 0.0 {
                **text = "Cooldown: READY".into();
                text_color.0=GREEN.into();
            } else {
                **text = format!("Cooldown: {:.2}", remaining);
                text_color.0=RED.into();
            }
        }
    }
}