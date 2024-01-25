use bevy::prelude::*;
use bevy::pbr::OpaqueRendererMethod;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::render::view::ColorGrading;
use crate::{Board, SpawnPlayerEvent, Player};
use super::*;


// slightly larger than 1.0 so the overlap prevents bleed through
const VOXEL_CUBE_SIZE: f32 = 1.0;
// const VOXEL_CUBE_MARGIN: f32 = 0.08;
                        const FOV:f32 = 120.;
const CAMERA3D_Z_POS:f32 = 20.;

pub(crate) fn spawn_voxel_map(
    board: Res<Board>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: ResMut<AssetServer>,
    mut ev: EventReader<SpawnPlayerEvent>,
    player_query: Query<(Entity, &Player)>,
) {
    // ..
    let texture_handle: Handle<Image> = asset_server.load("dirt.png");
    // ambient_light.color = Color::BLACK;


    let floor_material = materials.add(StandardMaterial {
        reflectance: 0.01,
        perceptual_roughness: 1.0,
        diffuse_transmission: 0.0,
        base_color_texture: Some(texture_handle.clone()),

        // normal_map_texture: None,
        metallic: 0.0,
        // parallax_mapping_method: ParallaxMappingMethod::Relief { max_steps: 3 },
        ior: 1.0,

        thickness: 1.0,
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

    // let margin = f32::EPSILON * 2.0;

    let shape = meshes.add(
        shape::Cube {
            size: VOXEL_CUBE_SIZE + f32::EPSILON,
        }
        .into(),
    );

    let bx = 0.0 - board.size.x as f32;
    let by = 0.0 - board.size.y as f32;

    let player_entity = player_query.single().0;

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
                    transform: Transform::from_xyz(
                        x as f32,
                        y as f32,
                        z as f32 - 1.0,
                    ),
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

            for SpawnPlayerEvent(position) in ev.read() {
                    ch
                    .spawn((
                        PlayerAvatar,
                        CreatureEntityRef(player_entity),
                        SpatialBundle {
                            transform: Transform::from_xyz(
                                position.x as f32,
                                position.y as f32,
                                0.,
                            ),
                            ..default()
                        },
                    ))
                    .with_children(|player| {
                        player.spawn((SpotLightBundle {
                            spot_light: SpotLight {
                                intensity: 950.,
                                range: 120.,
                                shadows_enabled: true,
                                color: Color::rgba_linear(0.8, 0.3, 0.05, 1.0),
                                outer_angle: 2.5,
                                inner_angle: 0.2,
                                ..default()
                            },
                            transform: Transform::from_xyz(0., 0., 0.25).looking_at(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.)),
                            ..default()
                        }, TorchMarker)).with_children( |torch| { 
                            torch.spawn((TorchSecondaryLightMarker, SpatialBundle::default())
                            ); 
                        });

                        // camera ...
                    
                        
                        player.spawn((
                            CameraMarker,
                            Camera3dBundle {
                                projection: Projection::Perspective(PerspectiveProjection { fov: FOV, ..default() }),
                                transform: Transform::from_xyz(0., 0., CAMERA3D_Z_POS)
                                    .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
                                camera_3d: Camera3d {
                                    clear_color: ClearColorConfig::Custom(Color::BLACK),
                                    screen_space_specular_transmission_steps: 3,
                                    screen_space_specular_transmission_quality: bevy::core_pipeline::core_3d::ScreenSpaceTransmissionQuality::Ultra,
                                    ..default()
                                },
                                tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
                                color_grading: ColorGrading {
                                    exposure: 0.5,
                                    gamma: 1.4,
                                    pre_saturation: 0.8,
                                    post_saturation: 0.6,
                                },
                                ..default()
                            },
                        ));
                    })
                    ;
            }
        });
}
