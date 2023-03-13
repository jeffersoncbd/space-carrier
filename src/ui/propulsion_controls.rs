use bevy::{
    prelude::Plugin as BevyPlugin,
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
    window::PrimaryWindow,
};

use crate::WinSize;

use super::resources;

#[derive(Component, Clone)]
struct Button {
    pub size: Vec2,
}

pub struct Plugin;
impl BevyPlugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(mouse_event_system);
    }
}

pub fn spawn_buttons(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    win_size: Res<WinSize>,
    game_icons: Res<resources::Icons>,
) {
    let background = Color::rgba_u8(30, 30, 30, 100);

    let button = Button {
        size: Vec2::new(40., 40.),
    };
    let margin = 10.;
    let x = -win_size.w / 2. + button.size.x / 2. + margin;
    let y = -win_size.h / 2. + button.size.y / 2. + margin;
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(button.size).into()).into(),
            material: materials.add(ColorMaterial::from(background.clone())),
            transform: Transform::from_translation(Vec3::new(x, y, 100.)),
            ..Default::default()
        })
        .insert(button.clone())
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: game_icons.atom.clone(),
                transform: Transform {
                    scale: Vec3::new(0.15, 0.15, 0.),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    let x = -win_size.w / 2. + button.size.x / 2. + margin * 2. + button.size.x;
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(button.size).into()).into(),
            material: materials.add(ColorMaterial::from(background.clone())),
            transform: Transform::from_translation(Vec3::new(x, y, 100.)),
            ..Default::default()
        })
        .insert(button.clone())
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: game_icons.fire.clone(),
                transform: Transform {
                    scale: Vec3::new(0.15, 0.15, 0.),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn mouse_event_system(
    mut buttons_query: Query<(&Handle<ColorMaterial>, &Transform, &Button), With<Button>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = windows_query.get_single() {
        for (color_material, transform, button) in buttons_query.iter_mut() {
            if let Some(position) = window.cursor_position() {
                let collistion = collide(
                    transform.translation,
                    button.size,
                    Vec3::new(
                        position.x - window.width() / 2.,
                        position.y - window.height() / 2.,
                        transform.translation.z,
                    ),
                    Vec2::new(1., 1.),
                );

                let color = if let Some(_) = collistion {
                    Color::rgba_u8(50, 50, 50, 200)
                } else {
                    Color::rgba_u8(30, 30, 30, 100)
                };
                materials.get_mut(color_material).unwrap().color = color;
            }
        }
    };
}
