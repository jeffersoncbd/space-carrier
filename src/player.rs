use bevy::{prelude::*, sprite::SpriteBundle};

use crate::{components::common::Velocity, resources::GameTextures};

#[derive(Resource)]
pub struct ShipCount(pub u8);

#[derive(Component)]
pub struct Player;

pub const MAX_VELOCITY: f32 = 100.;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_system)
            .add_system(keyboard_event_system);
    }
}

fn spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut ship_count: ResMut<ShipCount>,
) {
    if ship_count.0 < 1 {
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
            .insert(Player)
            .insert(Velocity { x: 0., y: 0. });
        ship_count.0 += 1;
    }
}

fn keyboard_event_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    match query.get_single_mut() {
        Ok(mut velocity) => {
            velocity.y = if keyboard.pressed(KeyCode::Space) {
                if velocity.y < MAX_VELOCITY {
                    velocity.y + (MAX_VELOCITY / 500.)
                } else {
                    MAX_VELOCITY
                }
            } else {
                if velocity.y < -0.1 {
                    velocity.y + (MAX_VELOCITY / 1000.)
                } else if velocity.y > 0.1 {
                    velocity.y - (MAX_VELOCITY / 1000.)
                } else {
                    0.
                }
            };
        }
        Err(error) => println!("{error}"),
    };
}
