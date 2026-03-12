use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::resources::GameResources;

#[derive(Component)]
pub(crate) struct Wall {
    pub(crate) health: f32,
}

pub(crate) struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_walls.after(crate::resources::load_resources));
    }
}

fn spawn_walls(mut commands: Commands, game_resources: Res<GameResources>) {
    spawn_wall(&mut commands, &game_resources, Vec2::new(100.0, 50.0));
    spawn_wall(&mut commands, &game_resources, Vec2::new(-150.0, -80.0));
}

fn spawn_wall(commands: &mut Commands, game_resources: &Res<GameResources>, position: Vec2) {
    let mut sprite = Sprite::from_image(game_resources.game_atlas.clone());
    sprite.rect = Some(Rect::new(730.0, 410.0, 730.0 + 66.0, 410.0 + 44.0)); // adjust rect to your atlas

    commands.spawn((
        Wall { health: 100.0 },
        sprite,
        Transform::from_translation(position.extend(0.0)),
        RigidBody::Fixed,
        Collider::cuboid(33.0, 22.0),
        ActiveEvents::COLLISION_EVENTS,

    ));
}