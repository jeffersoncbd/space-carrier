use bevy::{
    prelude::{Handle, Image, Resource},
    sprite::TextureAtlas,
};

#[derive(Resource)]
pub struct Textures {
    pub ship: Handle<Image>,
    pub flame: Handle<TextureAtlas>,
}
