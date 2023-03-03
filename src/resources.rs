use bevy::prelude::{Handle, Image, Resource};

#[derive(Resource)]
pub struct GameTextures {
    pub ship: Handle<Image>,
}
