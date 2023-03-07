use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
