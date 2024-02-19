use bevy::prelude::*;

// Health is used in both healthbars.rs and tanks.rs, so it is defined here
#[derive(Component)]
pub struct Health {
    pub value: u8,
}