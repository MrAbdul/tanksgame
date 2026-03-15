use bevy::prelude::{Bundle, Component};
use crate::bullet;

#[derive(Component)]
pub(crate) struct Health{
    pub(crate) health:f32,
}

#[derive(Component)]
pub(crate) struct TakesDamageFrom{
    pub(crate) damaging_bullets:Vec<bullet::BulletType>,
}

#[derive(Bundle)]
pub(crate) struct HealthBundle{
    pub(crate) health:Health,
    pub(crate) damaged_bullets:TakesDamageFrom
}

