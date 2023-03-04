use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct StarsPlugin;
impl Plugin for StarsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(spawn_system);
    }
}

fn spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(5., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform {
            translation: Vec3::new(30., 30., 0.),
            ..Default::default()
        },
        ..Default::default()
    });
}
