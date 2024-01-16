use bevy::prelude::*;
use dual_contouring::{DCBounds, DCInput};
use std::num::sqrt;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

struct SphereFunction;

impl DCInput for SphereFunction {
    fn get_bounds() -> DCBounds {
        DCBounds {
            x: (-10, 10),
            y: (-10, 10),
            z: (-10, 10),
        }
    }

    fn get_value(x: i64, y: i64, z: i64) -> f32 {
        sqrt(x * x + y * y + z * z) - 8.
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create camera in Bevy
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2., 1., 2.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Create light in Bevy
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(1., 3., 0.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add our mesh to Bevy
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Cube::default().into()),
        material: materials.add(StandardMaterial::default()),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}
