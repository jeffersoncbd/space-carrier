use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{thread_rng, Rng};

use crate::resources::WinSize;

#[derive(Resource)]
pub struct StarsCount(pub u32);

pub const MAX_STARS: u32 = 50;

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
    win_size: Res<WinSize>,
) {
    if stars_count.0 < MAX_STARS {
        let mut rng = thread_rng();

        // compute x/y
        let x_ref = win_size.w / 2.;
        let y_ref = win_size.h / 2.;
        let (x, y) = (
            rng.gen_range(-x_ref..x_ref) as f32,
            rng.gen_range(-y_ref..y_ref) as f32,
        );

        let star_radius = rng.gen_range(1..5) as f32;

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(star_radius, 6).into())
                .into(),
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
