use bevy::prelude::*;

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
        .run();
}
