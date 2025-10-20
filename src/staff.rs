use std::f32::consts::TAU;

use bevy::asset::RenderAssetUsages;
use bevy::color::palettes::css;
use bevy::math::ops::sin_cos;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

use crate::environment::FLOOR_HEIGHT;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub fn spawn_staff_mesh(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.05;
    let radial_variance = radius * 0.5;
    let height = 2.;
    let resolution = 6;
    let segments = 4;
    let horizontal_variance = height * 0.05;
    let mut rand = ChaCha8Rng::seed_from_u64(19878367467713);

    let mesh = generate_staff_mesh(
        radius,
        radial_variance,
        height,
        resolution,
        segments,
        horizontal_variance,
        &mut rand,
    );

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(Color::from(css::SADDLE_BROWN))),
        Transform::from_xyz(-2., height / 2. + FLOOR_HEIGHT / 2. + 0.5, 0.),
    ));
}

pub fn generate_staff_mesh(
    radius: f32,
    radial_variance: f32,
    height: f32,
    resolution: u32,
    segments: u32,
    horizontal_variance: f32,
    rand: &mut ChaCha8Rng,
) -> Mesh {
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

    // Bottom and Top variance X and Z must be known for cap placement

    // Bottom variance
    // Bottom radius should be a little smaller than top
    let bvr = rand.random_range((radius / 2. - radial_variance)..(radius / 2.));
    let bvx = rand.random::<f32>();
    let bvz = rand.random::<f32>();
    // Top variance
    let tvr = rand.random_range((radius - radial_variance)..radius);
    let tvx = rand.random::<f32>();
    let tvz = rand.random::<f32>();
    info!(
        "bvx: {:?}, bvz: {:?}, tvx: {:?}, tvy: {:?}",
        bvx, bvz, tvx, tvz
    );

    // rings

    for ring in 0..num_rings {
        // Radius with variance

        let (vr, vx, vz) = if ring == 0 {
            // Bottom variance radial, X, Z
            (bvr, bvx, bvz)
        } else if ring == num_rings - 1 {
            // Top variance radial, X, Z
            (tvr, tvx, tvz)
        } else {
            // New random variances X and Z
            (
                rand.random_range((radius - radial_variance)..radius),
                rand.random::<f32>(),
                rand.random::<f32>(),
            )
        };

        let offset = (horizontal_variance * vx, horizontal_variance * vz);
        let y = -half_height + ring as f32 * step_y;

        for segment in 0..=resolution {
            let theta = segment as f32 * step_theta;
            let (sin, cos) = sin_cos(theta);

            positions.push([vr * cos + offset.0, y, vr * sin + offset.1]);
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
        let (y, normal_y, winding, radial_variance, variance_offset) = if top {
            (
                half_height,
                1.,
                (1, 0),
                tvr,
                (horizontal_variance * tvx, horizontal_variance * tvz),
            )
        } else {
            (
                -half_height,
                -1.,
                (0, 1),
                bvr,
                (horizontal_variance * bvx, horizontal_variance * bvz),
            )
        };

        for i in 0..resolution {
            let theta = i as f32 * step_theta;
            let (sin, cos) = sin_cos(theta);

            positions.push([
                cos * radial_variance + variance_offset.0,
                y,
                sin * radial_variance + variance_offset.1,
            ]);
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
