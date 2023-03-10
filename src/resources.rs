use bevy::{
    prelude::{Handle, Image, Resource},
    sprite::TextureAtlas,
};

#[derive(Resource)]
pub struct GameTextures {
    pub ship: Handle<Image>,
    pub flame: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}
