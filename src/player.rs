use bevy::{
    prelude::{App, Commands, Plugin, Res},
    sprite::SpriteBundle,
};

use crate::resources::GameTextures;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_system);
    }
}

fn spawn_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    commands.spawn(SpriteBundle {
        texture: game_textures.ship.clone(),
        ..Default::default()
    });
}
