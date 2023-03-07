use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{thread_rng, Rng};

use crate::{components::common::Velocity, player::Player, resources::WinSize};

#[derive(Resource)]
pub struct StarsCount(pub u32);

#[derive(Component)]
pub struct Star {
    pub radius: f32,
}

pub const MAX_STARS: u32 = 50;

pub struct StarsPlugin;
impl Plugin for StarsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(spawn_system).add_system(stars_move_system);
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

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(star_radius, 6).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Star {
                radius: star_radius,
            });

        stars_count.0 += 1;
    }
}

fn stars_move_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    player_query: Query<&Velocity, With<Player>>,
    mut stars_query: Query<(Entity, &mut Transform, &Star), With<Star>>,
) {
    for velocity in player_query.iter() {
        for (entity, mut transform, star) in stars_query.iter_mut() {
            let translation = &mut transform.translation;
            translation.y -= velocity.y * star.radius;
        }
    }
}
