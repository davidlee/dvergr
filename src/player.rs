use crate::typical::graphics::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::view::ColorGrading;

const CAMERA3D_Z_POS: f32 = 20.;
const FOV: f32 = 120.;

#[derive(Component, Debug, Clone, Default)]
pub(crate) struct Player;

#[derive(Component, Debug, Clone, Default)]
pub(crate) struct Player2DMarker;

#[derive(Event, Debug)]
pub(crate) struct SpawnPlayerEvent(pub IVec3);

#[derive(Bundle, Debug)]
pub(crate) struct PlayerBundle {
    player: Player,
    creature: CreatureBundle,
    character: CharacterBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            player: Player,
            creature: CreatureBundle {
                locus: Locus {
                    position: IVec3::new(3, 3, 0),
                    ..default()
                },
                pace: Pace::default(),
                species: Species::Dwarf,
                ..default()
            },
            character: CharacterBundle {
                character: Character {
                    name: None,
                    level: CharacterLevel(1),
                    experience: 0,
                },
                ..default()
            },
        }
    }
}

pub(crate) fn spawn_player_and_3d_elements(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut ev: EventReader<SpawnPlayerEvent>,
    query: Query<(Entity, &mut BoardMarker)>,
) {
    let position = ev.read().next().unwrap().0;

    let player_bundle = PlayerBundle {
        creature: CreatureBundle {
            locus: Locus {
                position,
                ..default()
            },
            spatial: SpatialBundle {
                transform: Transform::from_xyz(position.x as f32, position.y as f32, 0.),
                ..default()
            },
            ..default()
        },
        ..default()
    };

    // let bx = 0. - (board.size.x / 2) as f32;
    // let by = 0. - (board.size.y / 2) as f32;

    // let cell_entity = board.cell_store.get(&position).unwrap();
    let (entity, _marker) = query.single();
    commands.entity(entity).with_children(|ch| {
        ch.spawn((
            Player2DMarker,
            SpatialBundle {
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            },
        ))
        .with_children(|ch| {
            let player_id = ch
                .spawn(player_bundle)
                .with_children(|player| {
                    player
                        .spawn((
                            SpotLightBundle {
                                spot_light: SpotLight {
                                    intensity: 950.,
                                    range: 120.,
                                    shadows_enabled: true,
                                    color: Color::rgba_linear(0.8, 0.3, 0.05, 1.0),
                                    outer_angle: 2.5,
                                    inner_angle: 0.2,
                                    ..default()
                                },
                                transform: Transform::from_xyz(0., 0., 0.25)
                                    .looking_at(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.)),
                                ..default()
                            },
                            TorchMarker,
                        ))
                        .with_children(|torch| {
                            torch.spawn((TorchSecondaryLightMarker, SpatialBundle::default()));
                        });

                    player.spawn((
                CameraMarker,
                Camera3dBundle {
                    projection: Projection::Perspective(PerspectiveProjection {
                        fov: FOV,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0., -20., 20.)
                        .looking_at(Vec3::new(0., 0., 0.), Vec3::new(0.,0.,0.)),
                    camera_3d: Camera3d {
                        clear_color: ClearColorConfig::Custom(Color::BLACK),
                        screen_space_specular_transmission_steps: 3,
                        screen_space_specular_transmission_quality:
                            bevy::core_pipeline::core_3d::ScreenSpaceTransmissionQuality::Ultra,
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
                .id();
            board.creature_store.insert(player_id, position);
        });
    });
}
