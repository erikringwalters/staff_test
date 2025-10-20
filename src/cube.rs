use bevy::mesh::Indices;
use bevy::{asset::RenderAssetUsages, color::palettes::css, mesh::PrimitiveTopology, prelude::*};

#[derive(Resource, Default, Debug)]
pub struct CubeNormals {
    positions: Vec<Vec3>,
    directions: Vec<Vec3>,
    origin: Vec3,
}

pub fn spawn_cube_mesh(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mut cube_normals: ResMut<CubeNormals>,
) {
    let mesh = generate_cube_mesh(&mut cube_normals);
    cube_normals.origin = vec3(1., 1., 1.);
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(Color::from(css::BLUE))),
        Transform::from_translation(cube_normals.origin),
    ));
}

pub fn generate_cube_mesh(cube_normals: &mut ResMut<CubeNormals>) -> Mesh {
    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    // Each array is an vec3(x, y, z) coordinate in local space.
    // The camera coordinate space is right-handed x-right, y-up, z-back. This means "forward" is -Z.
    // Meshes always rotate around their local vec3(0, 0, 0) when a rotation is applied to their Transform.
    // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
    cube_normals.positions = vec![
        // top (facing towards +y)
        vec3(-0.5, 0.5, -0.5), // vertex with index 0
        vec3(0.5, 0.5, -0.5),  // vertex with index 1
        vec3(0.5, 0.5, 0.5),   // etc. until 23
        vec3(-0.5, 0.5, 0.5),
        // bottom   (-y)
        vec3(-0.5, -0.5, -0.5),
        vec3(0.5, -0.5, -0.5),
        vec3(0.5, -0.5, 0.5),
        vec3(-0.5, -0.5, 0.5),
        // right    (+x)
        vec3(0.5, -0.5, -0.5),
        vec3(0.5, -0.5, 0.5),
        vec3(0.5, 0.5, 0.5), // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
        vec3(0.5, 0.5, -0.5),
        // left     (-x)
        vec3(-0.5, -0.5, -0.5),
        vec3(-0.5, -0.5, 0.5),
        vec3(-0.5, 0.5, 0.5),
        vec3(-0.5, 0.5, -0.5),
        // back     (+z)
        vec3(-0.5, -0.5, 0.5),
        vec3(-0.5, 0.5, 0.5),
        vec3(0.5, 0.5, 0.5),
        vec3(0.5, -0.5, 0.5),
        // forward  (-z)
        vec3(-0.5, -0.5, -0.5),
        vec3(-0.5, 0.5, -0.5),
        vec3(0.5, 0.5, -0.5),
        vec3(0.5, -0.5, -0.5),
    ];
    cube_normals.directions = vec![
        // Normals for the top side (towards +y)
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        // Normals for the bottom side (towards -y)
        vec3(0.0, -1.0, 0.0),
        vec3(0.0, -1.0, 0.0),
        vec3(0.0, -1.0, 0.0),
        vec3(0.0, -1.0, 0.0),
        // Normals for the right side (towards +x)
        vec3(1.0, 0.0, 0.0),
        vec3(1.0, 0.0, 0.0),
        vec3(1.0, 0.0, 0.0),
        vec3(1.0, 0.0, 0.0),
        // Normals for the left side (towards -x)
        vec3(-1.0, 0.0, 0.0),
        vec3(-1.0, 0.0, 0.0),
        vec3(-1.0, 0.0, 0.0),
        vec3(-1.0, 0.0, 0.0),
        // Normals for the back side (towards +z)
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 0.0, 1.0),
        // Normals for the forward side (towards -z)
        vec3(0.0, 0.0, -1.0),
        vec3(0.0, 0.0, -1.0),
        vec3(0.0, 0.0, -1.0),
        vec3(0.0, 0.0, -1.0),
    ];
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, cube_normals.positions.clone())
    // Set-up UV coordinates to point to the upper (V < 0.5), "dirt+grass" part of the texture.
    // Take a look at the custom image (assets/textures/array_texture.png)
    // so the UV coords will make more sense
    // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Assigning the UV coords for the top side.
            [0.0, 0.2],
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 0.2],
            // Assigning the UV coords for the bottom side.
            [0.0, 0.45],
            [0.0, 0.25],
            [1.0, 0.25],
            [1.0, 0.45],
            // Assigning the UV coords for the right side.
            [1.0, 0.45],
            [0.0, 0.45],
            [0.0, 0.2],
            [1.0, 0.2],
            // Assigning the UV coords for the left side.
            [1.0, 0.45],
            [0.0, 0.45],
            [0.0, 0.2],
            [1.0, 0.2],
            // Assigning the UV coords for the back side.
            [0.0, 0.45],
            [0.0, 0.2],
            [1.0, 0.2],
            [1.0, 0.45],
            // Assigning the UV coords for the forward side.
            [0.0, 0.45],
            [0.0, 0.2],
            [1.0, 0.2],
            [1.0, 0.45],
        ],
    )
    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, cube_normals.directions.clone())
    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    //
    // The first two defined triangles look like this (marked with the vertex indices,
    // and the axis), when looking down at the top (+y) of the cube:
    //   -Z
    //   ^
    // 0---1
    // |  /|
    // | / | -> +X
    // |/  |
    // 3---2
    //
    // The right face's (+x) triangles look like this, seen from the outside of the cube.
    //   +Y
    //   ^
    // 10--11
    // |  /|
    // | / | -> -Z
    // |/  |
    // 9---8
    //
    // The back face's (+z) triangles look like this, seen from the outside of the cube.
    //   +Y
    //   ^
    // 17--18
    // |\  |
    // | \ | -> +X
    // |  \|
    // 16--19
    .with_inserted_indices(Indices::U32(vec![
        0, 3, 1, 1, 3, 2, // triangles making up the top (+y) facing side.
        4, 5, 7, 5, 6, 7, // bottom (-y)
        8, 11, 9, 9, 11, 10, // right (+x)
        12, 13, 15, 13, 14, 15, // left (-x)
        16, 19, 17, 17, 19, 18, // back (+z)
        20, 21, 23, 21, 22, 23, // forward (-z)
    ]))
}

pub fn display_cube_vertex_normals(mut gizmos: Gizmos, mut cube_normals: ResMut<CubeNormals>) {
    for i in 0..cube_normals.positions.len() {
        let end = cube_normals.positions[i] + cube_normals.directions[i];
        draw_gizmos(&mut gizmos, &mut cube_normals, end, i);
    }
}

fn draw_gizmos(gizmos: &mut Gizmos, cube_normals: &mut ResMut<CubeNormals>, end: Vec3, i: usize) {
    gizmos.arrow(
        cube_normals.origin + cube_normals.positions[i],
        cube_normals.origin + end,
        css::WHITE,
    );
}
