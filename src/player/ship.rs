use bevy::{prelude::Plugin as BevyPlugin, prelude::*, sprite::SpriteBundle};

use crate::components::common::Velocity;

use super::resources::Textures;

#[derive(Component)]
pub struct Ship;

#[derive(Component)]
pub struct ShipData {
    pub velocity: Velocity,
    thruster_force: u64,
    mass: u64,
}

pub struct Plugin;
impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_event_system);
    }
}

pub fn spawn_system(mut commands: Commands, game_textures: Res<Textures>) {
    commands
        .spawn(SpriteBundle {
            texture: game_textures.ship.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 10.),
                scale: Vec3::new(0.3, 0.3, 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ship)
        .insert(ShipData {
            velocity: Velocity { x: 0., y: 0. },
            thruster_force: 10000,
            mass: 1000,
        });
}

fn keyboard_event_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut ShipData, With<Ship>>,
) {
    if let Ok(mut ship) = query.get_single_mut() {
        if keyboard.pressed(KeyCode::Space) {
            ship.velocity.y += (ship.thruster_force / ship.mass) as f32;
        }
    };
}
