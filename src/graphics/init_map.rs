use crate::marker_components::*;
use crate::Board;
use bevy::pbr::OpaqueRendererMethod;
use bevy::prelude::*;

// slightly larger than 1.0 so the overlap prevents bleed through
const VOXEL_CUBE_SIZE: f32 = 1.0;
// const VOXEL_CUBE_MARGIN: f32 = 0.08;

pub(crate) fn spawn_voxel_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    board: Res<Board>,
    asset_server: ResMut<AssetServer>,
) {
    // ..
    let texture_handle: Handle<Image> = asset_server.load("dirt.png");

    let floor_material = materials.add(StandardMaterial {
        reflectance: 0.01,
        perceptual_roughness: 1.0,
        diffuse_transmission: 0.0,
        base_color_texture: Some(texture_handle.clone()),
        ior: 1.0,
        specular_transmission: 0.2,
        attenuation_distance: 0.01,
        attenuation_color: Color::BLACK,
        emissive: Color::BLACK,
        alpha_mode: AlphaMode::Opaque,
        opaque_render_method: OpaqueRendererMethod::Deferred,
        fog_enabled: false,
        double_sided: true,
        ..default()
    });

    let shape = meshes.add(
        shape::Cube {
            size: VOXEL_CUBE_SIZE + f32::EPSILON,
        }
        .into(),
    );

    let bx = 0.0 - board.size.x as f32;
    let by = 0.0 - board.size.y as f32;

    commands.insert_resource(AmbientLight {
        color: Color::BLACK,
        brightness: 0.0,
    });

    commands
        .spawn((
            MapMarker,
            TransformBundle {
                local: Transform::from_xyz(bx, by, 0.),
                ..default()
            },
            Visibility::Inherited,
            InheritedVisibility::default(),
        ))
        .with_children(|ch| {
            for (ivec, _e) in board.cell_store.iter() {
                let [x, y, z] = ivec.to_array();

                // floor
                ch.spawn((PbrBundle {
                    mesh: shape.clone(),
                    material: floor_material.clone(),
                    transform: Transform::from_xyz(x as f32, y as f32, z as f32 - 1.0),
                    ..default()
                },));
            }

            for (ivec, _e) in board.wall_store.iter() {
                let [x, y, z] = ivec.to_array();

                // wall
                ch.spawn((PbrBundle {
                    mesh: shape.clone(),
                    material: floor_material.clone(),
                    transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                    ..default()
                },));
            }
        });
}
