use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use dual_contouring::{dc_meshing, DCBounds, DCInput};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
            }),
            WireframePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Default)]
struct SphereFunction;

impl DCInput for SphereFunction {
    fn get_bounds(&self) -> DCBounds {
        DCBounds {
            x: (-10, 10),
            y: (-10, 10),
            z: (-10, 10),
        }
    }

    fn get_value(&self, x: i32, y: i32, z: i32) -> f32 {
        f32::sqrt((x * x + y * y + z * z) as f32) - 8.
    }

    fn get_gradient(&self, x: i32, y: i32, z: i32) -> [f32; 3] {
        Vec3::new(x as f32, y as f32, z as f32).normalize().into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create camera in Bevy
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(16., 10., 6.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Create light in Bevy
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(1., 5., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let raw_output = dc_meshing(SphereFunction::default());
    let mesh = Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, raw_output.vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, raw_output.normals)
        .with_indices(Some(Indices::U32(raw_output.indices)));

    // Add our mesh to Bevy
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial::default()),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Wireframe,
    ));
}
