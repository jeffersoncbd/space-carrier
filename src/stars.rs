use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Resource)]
pub struct StarsCount(pub u32);

pub const MAX_STARS: u32 = 30;

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
    mut stars_count: ResMut<StarsCount>,
) {
    if stars_count.0 < MAX_STARS {
        let (x, y) = (25., 25.);
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(5., 6).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                ..Default::default()
            },
            ..Default::default()
        });
        stars_count.0 += 1;
    }
}
