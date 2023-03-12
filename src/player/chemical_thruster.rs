use bevy::{prelude::Plugin as BevyPlugin, prelude::*};

use super::resources::Textures;

#[derive(Component)]
pub struct Flame;

#[derive(Component)]
pub struct FlameTimer(pub Timer);
impl Default for FlameTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

pub struct Plugin;
impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(flame_animation_system)
            .add_system(keyboard_event_system);
    }
}

pub fn spawn_system(mut commands: Commands, game_textures: Res<Textures>) {
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: game_textures.flame.clone(),
            transform: Transform {
                translation: Vec3::new(0., -20., 9.),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(Flame)
        .insert(FlameTimer::default());
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
    mut query: Query<&mut Visibility, With<Flame>>,
) {
    if let Ok(mut flame) = query.get_single_mut() {
        if keyboard.pressed(KeyCode::Space) {
            *flame = Visibility::Visible;
        } else {
            *flame = Visibility::Hidden;
        }
    };
}
