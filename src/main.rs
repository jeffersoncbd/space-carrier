use bevy::prelude::*;
use player::ShipCount;
use resources::{GameTextures, WinSize};
use stars::StarsCount;
use ui::velocimeter::VelocimeterBlocks;

mod components;
mod player;
mod resources;
mod stars;
mod ui;

const SHIP_SPRITE: &str = "ship.png";

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Space Carrier".to_string(),
                mode: WindowMode::BorderlessFullscreen,
                height: 600.,
                width: 800.,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(player::PlayerPlugin)
        .add_plugin(stars::StarsPlugin)
        .add_plugin(ui::velocimeter::VelocimeterPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // insert textures
    commands.insert_resource(GameTextures {
        ship: asset_server.load(SHIP_SPRITE),
    });

    // capture window sizes
    let window = windows.get_primary_mut().unwrap();
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

    // set number of stars
    commands.insert_resource(StarsCount(0));

    // set number of ships
    commands.insert_resource(ShipCount(0));

    // set velocimeter blocks
    commands.insert_resource(VelocimeterBlocks(1));
}
