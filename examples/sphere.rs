/*
░█▀▀░█▀█░█░█░█▀▀░█▀▄░█▀▀░░░█▀▀░█░█░█▀█░█▄█░█▀█░█░░░█▀▀
░▀▀█░█▀▀░█▀█░█▀▀░█▀▄░█▀▀░░░█▀▀░▄▀▄░█▀█░█░█░█▀▀░█░░░█▀▀
░▀▀▀░▀░░░▀░▀░▀▀▀░▀░▀░▀▀▀░░░▀▀▀░▀░▀░▀░▀░▀░▀░▀░░░▀▀▀░▀▀▀

This example creates a mesh based on an SDF for a sphere.
It uses the bevy game engine to create a simple scene and
render the mesh.

Control + Mouse Drag orbits the camera.
Right Click + Mouse Drag pans the camera.
Scroll Wheel adjusts the zoom level.
*/

/*
░▀█▀░█▄█░█▀█░█▀█░█▀▄░▀█▀░█▀▀
░░█░░█░█░█▀▀░█░█░█▀▄░░█░░▀▀█
░▀▀▀░▀░▀░▀░░░▀▀▀░▀░▀░░▀░░▀▀▀
*/

use bevy::{
    pbr::wireframe::{Wireframe, WireframeColor, WireframePlugin},
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_infinite_grid::{
    GridShadowCamera, InfiniteGridBundle, InfiniteGridPlugin, InfiniteGridSettings,
};
use dual_contouring::{dc_meshing, DCBounds, DCInput};
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

/*
░█▀▄░█▀▀░█▀▀░▀█▀░█▀█░▀█▀░▀█▀░▀█▀░█▀█░█▀█░█▀▀
░█░█░█▀▀░█▀▀░░█░░█░█░░█░░░█░░░█░░█░█░█░█░▀▀█
░▀▀░░▀▀▀░▀░░░▀▀▀░▀░▀░▀▀▀░░▀░░▀▀▀░▀▀▀░▀░▀░▀▀▀
 */

#[derive(Default)]
struct SphereFunction;

impl DCInput for SphereFunction {
    fn get_bounds(&self) -> DCBounds {
        DCBounds {
            x: (-12, 12),
            y: (-12, 12),
            z: (-12, 12),
        }
    }

    fn get_value(&self, x: i32, y: i32, z: i32) -> f32 {
        f32::sqrt((x * x + y * y + z * z) as f32) - 8.5
    }

    fn get_gradient(&self, x: i32, y: i32, z: i32) -> [f32; 3] {
        Vec3::new(x as f32, y as f32, z as f32).normalize().into()
    }
}

/*
░█▀▀░█▀▀░▀█▀░█░█░█▀█
░▀▀█░█▀▀░░█░░█░█░█▀▀
░▀▀▀░▀▀▀░░▀░░▀▀▀░▀░░
*/

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create grid for referencing scale
    commands.spawn(InfiniteGridBundle::default());

    // Create camera in Bevy
    commands
        .spawn(Camera3dBundle::default())
        .insert(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(10., 10., 0.),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));

    // Create light in Bevy
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(1., 5., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Run the meshing algorithm on our sphere function
    let raw_output = dc_meshing(SphereFunction::default());

    // Define a Bevy mesh using the output from our meshing algorithm
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
        WireframeColor { color: Color::RED },
    ));
}

/*
░█▄█░█▀█░▀█▀░█▀█
░█░█░█▀█░░█░░█░█
░▀░▀░▀░▀░▀▀▀░▀░▀
*/

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
            LookTransformPlugin,
            OrbitCameraPlugin::default(),
            InfiniteGridPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}
