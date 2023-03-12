use bevy::{prelude::*, window::PrimaryWindow};

use crate::{WinSize, ICON_ATOM, ICON_FIRE};

mod propulsion_controls;
mod resources;

pub struct Plugins;
impl PluginGroup for Plugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let group = bevy::app::PluginGroupBuilder::start::<Self>();

        group
            .add(propulsion_controls::Plugin)
            .add(ConfigurationPlugin)
    }
}

struct ConfigurationPlugin;
impl Plugin for ConfigurationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup.in_base_set(CoreSet::First))
            .add_startup_system(propulsion_controls::spawn_buttons.after(setup));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
) {
    // insert icons
    commands.insert_resource(resources::Icons {
        atom: asset_server.load(ICON_ATOM),
        fire: asset_server.load(ICON_FIRE),
    });

    // capture window sizes
    let Ok(window) = windows_query.get_single() else {
            return;
        };
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });
}
