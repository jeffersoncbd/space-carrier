use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowMode},
};

mod components;
mod player;
mod scenario;
mod ui;

const SHIP_SPRITE: &str = "ship.png";
const FLAME_SHEET: &str = "p_flame_sheet.png";

const ICON_ATOM: &str = "icon_atom.png";
const ICON_FIRE: &str = "icon_fire.png";

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(ui::Plugins)
        .add_plugins(player::Plugins)
        .add_plugins(scenario::Plugins)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space Carrier".to_string(),
                mode: WindowMode::BorderlessFullscreen,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, windows_query: Query<&Window, With<PrimaryWindow>>) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // capture window sizes
    let Ok(window) = windows_query.get_single() else {
        return;
    };
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });
}
