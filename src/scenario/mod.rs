use bevy::{prelude::*, window::PrimaryWindow};

use crate::WinSize;
mod stars;

pub struct Plugins;
impl PluginGroup for Plugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let group = bevy::app::PluginGroupBuilder::start::<Self>();

        group.add(stars::Plugin).add(ConfigurationPlugin)
    }
}

struct ConfigurationPlugin;
impl Plugin for ConfigurationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup.in_base_set(CoreSet::First));
    }
}

fn setup(mut commands: Commands, windows_query: Query<&Window, With<PrimaryWindow>>) {
    // capture window sizes
    let Ok(window) = windows_query.get_single() else {
            return;
        };
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

    // set number of stars
    commands.insert_resource(stars::StarsCount(0));
}
