use bevy::prelude::{Component, IVec2};

#[derive(Component)]
pub struct Position {
    pub position: IVec2,
    pub prev_position: IVec2,
}
