use bevy::{prelude::Plugin as BevyPlugin, prelude::*, sprite::MaterialMesh2dBundle};

use crate::WinSize;

use super::resources;

pub struct Plugin;
impl BevyPlugin for Plugin {
    fn build(&self, _app: &mut bevy::prelude::App) {}
}

pub fn spawn_buttons(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    win_size: Res<WinSize>,
    game_icons: Res<resources::Icons>,
) {
    let background = Color::rgba_u8(30, 30, 30, 100);

    let button_size = (40., 40.);
    let margin = 10.;
    let x = -win_size.w / 2. + button_size.0 / 2. + margin;
    let y = -win_size.h / 2. + button_size.0 / 2. + margin;
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(button_size.0, button_size.1)).into())
                .into(),
            material: materials.add(ColorMaterial::from(background.clone())),
            transform: Transform::from_translation(Vec3::new(x, y, 100.)),
            ..Default::default()
        })
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

    let x = -win_size.w / 2. + button_size.0 / 2. + margin * 2. + button_size.0;
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(button_size.0, button_size.1)).into())
                .into(),
            material: materials.add(ColorMaterial::from(background.clone())),
            transform: Transform::from_translation(Vec3::new(x, y, 100.)),
            ..Default::default()
        })
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
