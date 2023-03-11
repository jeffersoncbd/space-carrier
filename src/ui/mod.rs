use bevy::prelude::{Commands, Plugin, PluginGroup};

mod propulsion_controls;
mod velocimeter;

pub struct Plugins;
impl PluginGroup for Plugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let group = bevy::app::PluginGroupBuilder::start::<Self>();

        group
            .add(propulsion_controls::Plugin)
            // .add(velocimeter::VelocimeterPlugin)
            .add(ConfigurationPlugin)
    }
}

struct ConfigurationPlugin;
impl Plugin for ConfigurationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(configuration_system);
    }
}

fn configuration_system(mut commands: Commands) {
    // set velocimeter blocks
    commands.insert_resource(velocimeter::VelocimeterBlocks(1));
}
