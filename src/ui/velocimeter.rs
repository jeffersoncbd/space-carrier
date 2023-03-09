use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    components::common::Velocity,
    player::{Player, MAX_VELOCITY},
    resources::WinSize,
};

#[derive(Resource)]
pub struct VelocimeterBlocks(pub u32);

#[derive(Component)]
pub struct VelocimeterBlock(pub u32);

const VELOCIMETER_BLOCK_SIZE: (f32, f32) = (20., 2.);

pub struct VelocimeterPlugin;
impl Plugin for VelocimeterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(spawn_system).add_system(despawn_system);
    }
}

fn spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut velocimeter_blocks: ResMut<VelocimeterBlocks>,
    win_size: Res<WinSize>,
    player_query: Query<&Velocity, With<Player>>,
) {
    if let Ok(velocity) = player_query.get_single() {
        let max_blocks = velocity.y / MAX_VELOCITY * 100.;
        if velocimeter_blocks.0 < max_blocks as u32 {
            let x = (-win_size.w / 2.) + (VELOCIMETER_BLOCK_SIZE.0 / 2.) + 10.;
            let y = (-win_size.h / 2.)
                + (VELOCIMETER_BLOCK_SIZE.1 / 2.)
                + (6. * velocimeter_blocks.0 as f32)
                + 4.;

            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(
                            shape::Quad::new(Vec2::new(
                                VELOCIMETER_BLOCK_SIZE.0,
                                VELOCIMETER_BLOCK_SIZE.1,
                            ))
                            .into(),
                        )
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::YELLOW_GREEN)),
                    transform: Transform::from_translation(Vec3::new(x, y, 100.)),
                    ..Default::default()
                })
                .insert(VelocimeterBlock(velocimeter_blocks.0));

            velocimeter_blocks.0 += 1;
        }
    }
}

fn despawn_system(
    mut commands: Commands,
    mut velocimeter_blocks: ResMut<VelocimeterBlocks>,
    velocimeter_query: Query<(Entity, &VelocimeterBlock), With<VelocimeterBlock>>,
    player_query: Query<&Velocity, With<Player>>,
) {
    if let Ok(velocity) = player_query.get_single() {
        let max_blocks = velocity.y / MAX_VELOCITY * 100.;
        for (entity, block) in velocimeter_query.iter() {
            if block.0 > max_blocks as u32 {
                commands.entity(entity).despawn();
                velocimeter_blocks.0 -= 1;
            }
        }
    }
}
