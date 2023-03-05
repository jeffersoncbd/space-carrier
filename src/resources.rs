use bevy::prelude::{Handle, Image, Resource};

#[derive(Resource)]
pub struct GameTextures {
    pub ship: Handle<Image>,
}

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}
