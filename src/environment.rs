use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::{
    asset_loader::SceneAssets, cone::spawn_cone_mesh, cube::spawn_cube_mesh,
    cylinder::spawn_cylinder_mesh, staff::spawn_staff_mesh,
};

const SUN_DISTANCE: f32 = 100.;
pub const FLOOR_LENGTH: f32 = 40.;
pub const FLOOR_HEIGHT: f32 = 1.;
pub const FLOOR_SIZE: Vec3 = vec3(FLOOR_LENGTH, FLOOR_HEIGHT, FLOOR_LENGTH);

#[derive(Component)]
pub struct Floor;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_environment);
    }
}

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scene_assets: Res<SceneAssets>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    commands.spawn((
        Name::new("Sun"),
        DirectionalLight {
            illuminance: 2500.,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(SUN_DISTANCE, SUN_DISTANCE * 0.5, SUN_DISTANCE)
            .looking_at(Vec3::ZERO, Dir3::Y),
    ));

    commands.spawn((
        Name::new("Floor"),
        Floor,
        Mesh3d(meshes.add(Cuboid::new(FLOOR_SIZE.x, FLOOR_SIZE.y, FLOOR_SIZE.z))),
        MeshMaterial3d(debug_material.clone()),
        Transform::from_translation(Vec3::ZERO),
    ));

    commands.spawn((
        Name::new("Laura"),
        SceneRoot(scene_assets.laura.clone()),
        MeshMaterial3d(materials.add(Color::from(css::DARK_GREEN))),
        Transform::from_xyz(0., FLOOR_HEIGHT / 2., 0.),
        Visibility::default(),
    ));

    spawn_cube_mesh(&mut commands, &mut meshes, &mut materials);
    spawn_cone_mesh(&mut commands, &mut meshes, &mut materials);
    spawn_cylinder_mesh(&mut commands, &mut meshes, &mut materials);
    spawn_staff_mesh(&mut commands, &mut meshes, &mut materials);
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
