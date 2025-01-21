use bevy::prelude::*;

#[derive(Component)]
pub struct Pool {
    pub health: f32,
    pub max_health: f32,
    pub damage: f32,
}

pub(super) fn plugin(app: &mut App) {}
