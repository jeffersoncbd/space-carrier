use bevy::prelude::*;
use resources::GameTextures;

mod player;
mod resources;

const SHIP_SPRITE: &str = "ship.png";

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Space Carrier".to_string(),
                mode: WindowMode::Fullscreen,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let game_texture = GameTextures {
        ship: asset_server.load(SHIP_SPRITE),
    };

    commands.insert_resource(game_texture);
}
