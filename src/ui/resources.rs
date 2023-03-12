use bevy::prelude::{Handle, Image, Resource};

#[derive(Resource)]
pub struct Icons {
    pub atom: Handle<Image>,
    pub fire: Handle<Image>,
}
