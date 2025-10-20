use std::f32::consts::TAU;

use bevy::asset::RenderAssetUsages;
use bevy::color::palettes::css;
use bevy::math::ops::sin_cos;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
// use rand::SeedableRng;
// use rand_chacha::ChaCha8Rng;

use crate::environment::FLOOR_HEIGHT;

pub fn spawn_crystal_mesh(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.5;
    let radial_variance = radius * 0.5;
    let height = 1.;
    let resolution = 6;
    // let horizontal_variance = height * 0.05;
    // let mut rand = ChaCha8Rng::seed_from_u64(19878367467713);

    let mesh = generate_crystal_mesh(radius, radial_variance, resolution);

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(Color::from(css::SKY_BLUE))),
        Transform::from_xyz(-1., height / 2. + FLOOR_HEIGHT / 2., -1.),
    ));
}

fn generate_crystal_mesh(radius: f32, height: f32, resolution: u32) -> Mesh {
    let segments = 1;
    let half_height = height / 2.;
    debug_assert!(resolution > 2);
    debug_assert!(resolution > 0);

    let num_rings = segments + 1;
    let num_vertices = resolution * 2 + num_rings * (resolution + 1);
    let num_faces = resolution * (num_rings - 2);
    let num_indices = (2 * num_faces + 2 * (resolution - 1) * 2) * 3;

    let mut positions = Vec::with_capacity(num_vertices as usize);
    let mut normals = Vec::with_capacity(num_vertices as usize);
    let mut uvs = Vec::with_capacity(num_vertices as usize);
    let mut indices = Vec::with_capacity(num_indices as usize);

    let step_theta = TAU / resolution as f32;
    let step_y = 2.0 * half_height / segments as f32;

    // rings

    for ring in 0..num_rings {
        let y = -half_height + ring as f32 * step_y;

        for segment in 0..=resolution {
            let theta = segment as f32 * step_theta;
            let (sin, cos) = sin_cos(theta);

            positions.push([radius * cos, y, radius * sin]);
            normals.push([cos, 0., sin]);
            uvs.push([
                segment as f32 / resolution as f32,
                ring as f32 / segment as f32,
            ]);
        }
    }

    // barrel skin

    for i in 0..segments {
        let ring = i * (resolution + 1);
        let next_ring = (i + 1) * (resolution + 1);

        for j in 0..resolution {
            indices.extend_from_slice(&[
                ring + j,
                next_ring + j,
                ring + j + 1,
                next_ring + j,
                next_ring + j + 1,
                ring + j + 1,
            ]);
        }
    }

    // caps
    let mut build_cap = |top: bool| {
        let offset = positions.len() as u32;
        let (y, normal_y, winding) = if top {
            (half_height, 1., (1, 0))
        } else {
            (-half_height, -1., (0, 1))
        };

        for i in 0..resolution {
            let theta = i as f32 * step_theta;
            let (sin, cos) = sin_cos(theta);

            positions.push([cos * radius, y, sin * radius]);
            normals.push([0.0, normal_y, 0.0]);
            uvs.push([0.5 * (cos + 1.0), 1.0 - 0.5 * (sin + 1.0)]);
        }

        for i in 1..(resolution - 1) {
            indices.extend_from_slice(&[offset, offset + i + winding.0, offset + i + winding.1]);
        }
    };

    build_cap(true);
    build_cap(false);

    // Assume anchor is at midpoint. No need for vertex position offsets

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_indices(Indices::U32(indices))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
}
