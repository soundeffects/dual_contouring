use glam::i32::IVec3;
use std::collections::HashMap;

pub struct DCBounds {
    pub x: (i32, i32),
    pub y: (i32, i32),
    pub z: (i32, i32),
}

pub trait DCInput {
    fn get_bounds(&self) -> DCBounds;
    fn get_value(&self, x: i32, y: i32, z: i32) -> f32;
    fn get_gradient(&self, x: i32, y: i32, z: i32) -> [f32; 3];
}

pub struct DCOutput {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

pub fn dc_meshing(input: impl DCInput) -> DCOutput {
    let bounds = input.get_bounds();

    // Validate bounds are positive in debug builds
    debug_assert!(
        bounds.x.1 - bounds.x.0 - 1 > 0
            && bounds.y.1 - bounds.y.0 - 1 > 0
            && bounds.z.1 - bounds.z.0 - 1 > 0
    );

    let mut vertex_buffer = Vec::<[f32; 3]>::new();
    let mut normal_buffer = Vec::<[f32; 3]>::new();
    let mut index_list = Vec::<u32>::new();
    let mut vertex_map = HashMap::<IVec3, u32>::new();

    let planes = [
        (IVec3::X, IVec3::Y),
        (IVec3::X, IVec3::Z),
        (IVec3::Y, IVec3::Z),
    ];

    for x in bounds.x.0..(bounds.x.1 - 1) {
        for y in bounds.y.0..(bounds.y.1 - 1) {
            for z in bounds.z.0..(bounds.z.1 - 1) {
                if let Some(vertex) = dc_place_vertex(&input, x, y, z) {
                    let position = IVec3::new(x, y, z);
                    vertex_map.insert(position, vertex_buffer.len() as u32);
                    vertex_buffer.push(vertex);
                    normal_buffer.push(input.get_gradient(x, y, z));

                    for (basis_1, basis_2) in planes {
                        check_quad(
                            &input,
                            &vertex_map,
                            position,
                            basis_1,
                            basis_2,
                            &mut index_list,
                        );
                    }
                }
            }
        }
    }

    DCOutput {
        vertices: vertex_buffer,
        normals: normal_buffer,
        indices: index_list,
    }
}

fn dc_place_vertex(input: &impl DCInput, x: i32, y: i32, z: i32) -> Option<[f32; 3]> {
    let points = [
        input.get_value(x, y, z),
        input.get_value(x + 1, y, z),
        input.get_value(x + 1, y + 1, z),
        input.get_value(x, y + 1, z),
        input.get_value(x, y, z + 1),
        input.get_value(x + 1, y, z + 1),
        input.get_value(x + 1, y + 1, z + 1),
        input.get_value(x, y + 1, z + 1),
    ];

    for index in 0..7 {
        if sign_change(points[index], points[index + 1]) {
            return Some([x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5]);
        }
    }
    None
}

fn sign_change(a: f32, b: f32) -> bool {
    a.max(b) > 0. && a.min(b) < 0.
}

fn check_quad(
    input: &impl DCInput,
    vertex_map: &HashMap<IVec3, u32>,
    position: IVec3,
    basis_1: IVec3,
    basis_2: IVec3,
    mut index_list: &mut Vec<u32>,
) {
    let top_left = position - basis_1;
    let top_right = position;
    let bottom_left = position - basis_1 - basis_2;
    let bottom_right = position - basis_2;

    if (position + basis_1 + basis_2).min_element() > 0
        && vertex_map.contains_key(&top_left)
        && vertex_map.contains_key(&bottom_left)
        && vertex_map.contains_key(&bottom_right)
    {
        create_quad(
            &mut index_list,
            vertex_map[&top_left],
            vertex_map[&top_right],
            vertex_map[&bottom_left],
            vertex_map[&bottom_right],
            input.get_value(position.x, position.y, position.z) > 0.,
        );
    }
}

fn create_quad(
    index_list: &mut Vec<u32>,
    top_left: u32,
    top_right: u32,
    bottom_left: u32,
    bottom_right: u32,
    clockwise: bool,
) {
    if clockwise {
        // Top left triangle
        index_list.push(top_left);
        index_list.push(top_right);
        index_list.push(bottom_left);

        // Bottom right triangle
        index_list.push(bottom_right);
        index_list.push(bottom_left);
        index_list.push(top_right);
    } else {
        // Top left triangle
        index_list.push(top_left);
        index_list.push(bottom_left);
        index_list.push(top_right);

        // Bottom right triangle
        index_list.push(bottom_right);
        index_list.push(top_right);
        index_list.push(bottom_left);
    }
}
