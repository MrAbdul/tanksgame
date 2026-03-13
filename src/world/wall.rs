use crate::resources::GameResources;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d};
use bevy_rapier2d::prelude::*;

const WALL_MAX_HEALTH: f32 = 100.0;
pub(crate) const WALL_SIZE: Vec2 = Vec2::new(66.0, 44.0);
const WALL_ATLAS_RECT_PX: Vec4 = Vec4::new(730.0, 410.0, 66.0, 44.0);

#[derive(Component)]
pub(crate) struct Wall {
    pub(crate) health: f32,
}

#[derive(Component)]
pub(crate) struct WallFlash {
    pub(crate) timer: Timer,
}
//we add shader type here to denote a GPU safe memory layout to be sent to the shader
#[derive(Clone, Copy, Debug, ShaderType)]
pub(crate) struct WallMaterialParams {
    //this will be the rect that slices the wall from the atlas
    pub(crate) atlas_rect_px: Vec4,
    //a vec 4 that has x as the health ration 0..1 and the flash amount 0..1, z and w unused for now
    pub(crate) health_flash: Vec4,
}

//the custome material that is the actual shader inputs, it contains everything the wall shader will need
//AsBindGroup converts the struct into shader bindings
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(crate) struct WallMaterial {
    //these are the data passed to the shader in slots
    #[uniform(0)]
    pub(crate) params: WallMaterialParams,
    #[texture(1)]
    #[sampler(2)]
    pub(crate) atlas: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    pub(crate) crack_mask: Handle<Image>,
}

impl Material2d for WallMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/wall_damage.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub(crate) fn spawn_wall(
    commands: &mut Commands,
    game_resources: &Res<GameResources>,
    crack_mask: &Handle<Image>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<WallMaterial>>,
    position: Vec2,
) {
    let mesh = meshes.add(Rectangle::new(WALL_SIZE.x, WALL_SIZE.y));
    // Important: each wall gets its own material handle,
    // so each wall can have independent health/flash visuals.
    let material = materials.add(WallMaterial {
        params: WallMaterialParams {
            atlas_rect_px: WALL_ATLAS_RECT_PX,
            health_flash: Vec4::new(1.0, 0.0, 0.0, 0.0),
        },
        atlas: game_resources.game_atlas.clone(),
        crack_mask: crack_mask.clone(),
    });

    commands.spawn((
        Wall { health: WALL_MAX_HEALTH },
        WallFlash { timer: Timer::from_seconds(0.1, TimerMode::Once) },
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(position.extend(0.0)),
        RigidBody::Fixed,
        Collider::cuboid(WALL_SIZE.x * 0.5, WALL_SIZE.y * 0.5),
        ActiveEvents::COLLISION_EVENTS,
    ));
}

pub(crate) fn sync_wall_materials(
    time: Res<Time>,
    mut materials: ResMut<Assets<WallMaterial>>,
    mut walls: Query<(&Wall, &mut WallFlash, &MeshMaterial2d<WallMaterial>)>,
) {
    for (wall, mut flash, material_handle) in &mut walls {
        flash.timer.tick(time.delta());
        let flash_amount = if flash.timer.is_finished() {
            0.0
        } else {
            let duration = flash.timer.duration().as_secs_f32().max(0.0001);
            1.0 - (flash.timer.elapsed_secs() / duration)
        };
        let health_ratio = (wall.health / WALL_MAX_HEALTH).clamp(0.0, 1.0);
        if let Some(mat) = materials.get_mut(&material_handle.0) {
            mat.params.health_flash = Vec4::new(health_ratio, flash_amount, 0.0, 0.0);
        }
    }
}