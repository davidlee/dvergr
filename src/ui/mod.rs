use crate::state::AppInitEvent;
use crate::AppState;
use bevy::prelude::*;
use bevy_pancam::PanCam;

/*
pallette:

006466 065A60 0B525B 144552 1B3A4B
212F45 272640 312244 3E1F47 4D194D
*/

#[derive(Component, Debug)]
pub struct MapViewPanel;

#[derive(Component, Debug)]
pub struct MapViewContainer;

pub fn spawn_layout_shim(mut ev_writer: EventWriter<AppInitEvent>) {
    ev_writer.send(AppInitEvent::SetAppState(AppState::InitTileMap));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(PanCam {
        min_scale: 0.1,
        max_scale: Some(2.),
        ..default()
    });
}

//
// UI Layout
//
#[allow(dead_code)]
pub fn __spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_writer: EventWriter<AppInitEvent>,
) {
    println!("THIS IS WHERE WE LOAD UI");

    let heading_style = TextStyle {
        font: asset_server.load("font/BigBlueTerminalPlus.ttf"),
        font_size: 22.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };

    #[allow(unused_variables)]
    let text_style = TextStyle {
        font: asset_server.load("font/BigBlueTerminalPlus.ttf"),
        font_size: 14.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };

    commands
        // the outer container
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::hex("272640").unwrap()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                // top full width menu bar
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Px(50.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::hex("1B3A4B").unwrap()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("GAME MENU", heading_style.clone()));
                });
            parent
                // middle vertical third, split into sidebar and main
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Sidebar
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                min_width: Val::Px(200.),
                                height: Val::Percent(100.),
                                // for contained text
                                // align_items: AlignItems::Start,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::hex("144552").unwrap()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("Sidebar", heading_style.clone()));
                        });
                    // Main Content Container
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_grow: 100.,
                                height: Val::Percent(100.),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::hex("000000").unwrap()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    MapViewPanel,
                                    NodeBundle {
                                        // Map Area
                                        style: Style {
                                            flex_grow: 100.,
                                            height: Val::Percent(70.),
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        background_color: BackgroundColor(
                                            Color::rgba_u8(10, 45, 75, 45),
                                            // Color::hex("212F45").unwrap(),
                                        ),
                                        ..Default::default()
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "ASCII Map Goes Here",
                                            text_style.clone(),
                                        ),
                                        MapViewContainer,
                                    ));
                                    // MAP GOES HERE
                                    // parent.spawn(MapViewContainer);
                                });

                            parent
                                .spawn(NodeBundle {
                                    // Console
                                    style: Style {
                                        flex_grow: 100.,
                                        height: Val::Percent(30.),
                                        ..default()
                                    },
                                    background_color: BackgroundColor(
                                        Color::hex("272640").unwrap(),
                                    ),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Console Output",
                                        text_style.clone(),
                                    ));
                                });
                        });
                });
            parent
                // full width footer
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        bottom: Val::Px(0.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::hex("1B3A4B").unwrap()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Footer", heading_style.clone()));
                });
        });

    ev_writer.send(AppInitEvent::SetAppState(AppState::InitTileMap));
}
