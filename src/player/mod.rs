use bevy::prelude::*;

use crate::{FLAME_SHEET, SHIP_SPRITE};
mod resources;

pub mod chemical_thruster;
pub mod ship;

pub struct Plugins;
impl PluginGroup for Plugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let group = bevy::app::PluginGroupBuilder::start::<Self>();

        group
            .add(chemical_thruster::Plugin)
            .add(ship::Plugin)
            .add(ConfigurationPlugin)
    }
}

struct ConfigurationPlugin;
impl Plugin for ConfigurationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup.in_base_set(CoreSet::First))
            .add_startup_system(ship::spawn_system.after(setup))
            .add_startup_system(chemical_thruster::spawn_system.after(setup));
    }
}

fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    // create flame texture atlas
    let texture_handle = asset_server.load(FLAME_SHEET);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(8., 16.), 2, 1, None, None);

    // insert textures
    commands.insert_resource(resources::Textures {
        ship: asset_server.load(SHIP_SPRITE),
        flame: texture_atlases.add(texture_atlas),
    });
}
