use bevy::{prelude::*, sprite::SpriteBundle};

use crate::{components::common::Velocity, resources::GameTextures};

#[derive(Resource)]
pub struct ShipCount(pub u8);

#[derive(Resource)]
pub struct FlameCount(pub u8);

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Flame;

#[derive(Component)]
pub struct FlameTimer(pub Timer);
impl Default for FlameTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

pub const MAX_VELOCITY: f32 = 100.;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ship_system)
            .add_system(flame_spawn_system)
            .add_system(flame_animation_system)
            .add_system(keyboard_event_system);
    }
}

fn spawn_ship_system(
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

fn flame_spawn_system(
    mut commands: Commands,
    mut flame_count: ResMut<FlameCount>,
    keyboard: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<Entity, With<Flame>>,
) {
    if keyboard.pressed(KeyCode::Space) {
        if flame_count.0 < 1 {
            commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: game_textures.flame.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., -20., 9.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Flame)
                .insert(FlameTimer::default());
            flame_count.0 += 1;
        }
    } else {
        if let Ok(entity) = query.get_single() {
            commands.entity(entity).despawn();
            flame_count.0 -= 1;
        }
    };
}

fn flame_animation_system(
    time: Res<Time>,
    mut query: Query<(&mut FlameTimer, &mut TextureAtlasSprite), With<Flame>>,
) {
    if let Ok((mut timer, mut sprite)) = query.get_single_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index = if sprite.index == 0 { 1 } else { 0 }
        }
    }
}

fn keyboard_event_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
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
    };
}
