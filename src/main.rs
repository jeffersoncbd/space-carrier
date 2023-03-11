use bevy::prelude::*;
use player::{FlameCount, ShipCount};
use resources::{GameIcons, GameTextures, WinSize};
use stars::StarsCount;

mod components;
mod player;
mod resources;
mod stars;
mod ui;

const SHIP_SPRITE: &str = "ship.png";
const FLAME_SHEET: &str = "p_flame_sheet.png";

const ICON_ATOM: &str = "icon_atom.png";
const ICON_FIRE: &str = "icon_fire.png";

fn main() {
    App::new()
        .add_startup_system(setup_system)
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
        .add_plugins(ui::Plugins)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(stars::StarsPlugin)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // create flame texture atlas
    let texture_handle = asset_server.load(FLAME_SHEET);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(8., 16.), 2, 1, None, None);

    // insert icons
    commands.insert_resource(GameIcons {
        atom: asset_server.load(ICON_ATOM),
        fire: asset_server.load(ICON_FIRE),
    });

    // insert textures
    commands.insert_resource(GameTextures {
        ship: asset_server.load(SHIP_SPRITE),
        flame: texture_atlases.add(texture_atlas),
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
    commands.insert_resource(FlameCount(0));
}
