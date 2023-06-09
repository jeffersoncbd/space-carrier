use bevy::{prelude::Plugin as BevyPlugin, prelude::*, sprite::MaterialMesh2dBundle};
use rand::{thread_rng, Rng};

use crate::{
    player::ship::{Ship, ShipData},
    WinSize,
};

#[derive(Resource)]
pub struct StarsCount(pub u32);

#[derive(Component)]
pub struct Star {
    pub radius: f32,
}

pub const MAX_STARS: u32 = 300;
const MARGIN_TO_DESPAWN: f32 = 10.;

pub struct Plugin;
impl BevyPlugin for Plugin {
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

        let star_radius = if stars_count.0 <= (MAX_STARS as f32 * 0.5) as u32 {
            0.9
        } else if stars_count.0 <= (MAX_STARS as f32 * 0.7) as u32 {
            1.
        } else if stars_count.0 <= (MAX_STARS as f32 * 0.8) as u32 {
            1.1
        } else if stars_count.0 <= (MAX_STARS as f32 * 0.9) as u32 {
            1.2
        } else if stars_count.0 <= (MAX_STARS as f32 * 0.95) as u32 {
            1.3
        } else if stars_count.0 <= (MAX_STARS as f32 * 0.99) as u32 {
            1.4
        } else {
            1.7
        };

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
    win_size: Res<WinSize>,
    ship_query: Query<&ShipData, With<Ship>>,
    mut stars_query: Query<(&mut Transform, &Star), With<Star>>,
) {
    if let Ok(ship) = ship_query.get_single() {
        for (mut transform, star) in stars_query.iter_mut() {
            let translation = &mut transform.translation;
            translation.y -= (ship.velocity.y / 1000000.) * (star.radius * 10.);

            // recompute x/y
            if translation.y < -win_size.h / 2. + MARGIN_TO_DESPAWN {
                let mut rng = thread_rng();
                let x_ref = win_size.w / 2.;
                let (x, y) = (
                    rng.gen_range(-x_ref..x_ref) as f32,
                    win_size.h / 2. + MARGIN_TO_DESPAWN,
                );
                translation.y = y;
                translation.x = x;
            }
        }
    }
}
