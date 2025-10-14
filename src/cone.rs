use std::f32::consts::TAU;

use bevy::asset::RenderAssetUsages;
use bevy::color::palettes::css;
use bevy::math::ops::sin_cos;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

use crate::environment::FLOOR_HEIGHT;

pub fn spawn_cone_mesh(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let height = 1.;
    let radius = 0.5;
    let resolution = 64;
    let mesh = generate_cone_mesh(height, radius, resolution);
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(Color::from(css::PINK))),
        Transform::from_xyz(1., height / 2. + FLOOR_HEIGHT / 2., -1.),
    ));
}

fn generate_cone_mesh(height: f32, radius: f32, resolution: u32) -> Mesh {
    // referenced from bevy source code: crates/bevy_mesh/src/primitives/dim3/cone.rs
    let half_height = height / 2.;

    let num_vertices = resolution as usize * 2 + 1;
    let num_indices = resolution as usize * 6 - 6;

    let mut positions = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);
    let mut uvs = Vec::with_capacity(num_vertices);
    let mut indices = Vec::with_capacity(num_indices);

    // Tip of the cone
    positions.push([0., half_height, 0.]);

    // This is an invalid shader for the tip so the shading of the cone isn't affected.
    normals.push([0.; 3]);

    // UV's of a cone are in polar coordinates. Imagine projecting a circle texture from above.
    // The center of the texutre is at the tip of the cone.
    uvs.push([0.5; 2]);

    // Side of cone

    // Vertex normals are perpendicular to surface
    //
    // We get the slope of the normal
    // then use that slope to calculate the "multiplicative inverse"
    // of the length
    // of a vector
    // in the direction of a normal.
    // We use this for efficient normalization.
    let normal_slope = radius / height;

    // Equivalent to Vec2::new(1.0, slope).length().recip()
    let normalization_factor = (1.0 + normal_slope * normal_slope).sqrt().recip();

    // Change in angle at each step
    let step_theta = TAU / resolution as f32;

    // Bottom vertices for the lateral surfaces
    for segment in 0..resolution {
        // Angle for the segment
        let theta = segment as f32 * step_theta;
        let (sin, cos) = sin_cos(theta);

        // Vertex normal perpendicular to the side
        let normal = Vec3::new(cos, normal_slope, sin) * normalization_factor;

        positions.push([radius * cos, -half_height, radius * sin]);
        normals.push(normal.to_array());
        uvs.push([0.5 + cos * 0.5, 0.5 + sin * 0.5]);
    }

    // Add indices for lateral surface.
    // Each triangle is made by connecting two base vertices to the tip.
    for j in 1..resolution {
        indices.extend_from_slice(&[0, j + 1, j]);
    }

    // Close the lateral surface by stitching the first and last base vertices.
    indices.extend_from_slice(&[0, 1, resolution]);

    // Base of cone
    let index_offset = positions.len() as u32;

    // Vertices of the base
    for i in 0..resolution {
        let theta = i as f32 * step_theta;
        let (sin, cos) = sin_cos(theta);

        positions.push([cos * radius, -half_height, sin * radius]);
        normals.push([0.0, -1.0, 0.0]);
        uvs.push([0.5 * (cos + 1.0), 1.0 - 0.5 * (sin + 1.0)]);
    }

    // Add triangle indices for base
    for i in 1..(resolution - 1) {
        indices.extend_from_slice(&[index_offset, index_offset + i, index_offset + i + 1]);
    }

    // Note: in original ConeMeshBuilder, user can specify where the anchor is.
    // The anchor determines the Y offset for vertices to match anchor.
    // Here we will assume the anchor is the midpoint, so no offset needed.

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_indices(Indices::U32(indices))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
}
