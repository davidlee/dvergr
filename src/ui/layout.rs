use crate::graphics::typical::*;
use crate::typical::*;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MapViewPanel;

//
// UI Layout
//

const COLORS: [Color; 4] = [
    //
    Color::rgb(0.05, 0.15, 0.11), // header
    Color::rgb(0.0, 0.2, 0.4),    // footer
    Color::rgb(0.0, 0.0, 0.15),   // side
    Color::rgb(0.0, 0.05, 0.1),   // term
                                  //
];

#[allow(dead_code)]
pub fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_writer: EventWriter<AppInitEvent>,
) {
    info!("this is where we load the UI for real, disabled because I don't know how to mount the map into it");

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
            background_color: BackgroundColor(Color::NONE),
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
                    background_color: BackgroundColor(COLORS[1]),
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
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Sidebar
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                min_width: Val::Px(200.),
                                height: Val::Percent(100.),
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(COLORS[2]),
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
                            background_color: BackgroundColor(Color::NONE),
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
                                        background_color: BackgroundColor(Color::NONE),
                                        ..default()
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "ASCII Map Goes Here",
                                        text_style.clone(),
                                    ));
                                });

                            parent
                                .spawn(NodeBundle {
                                    // Console
                                    style: Style {
                                        flex_grow: 100.,
                                        height: Val::Percent(30.),
                                        ..default()
                                    },
                                    background_color: BackgroundColor(COLORS[3]),
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
                    background_color: BackgroundColor(COLORS[1]),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Footer", heading_style.clone()));
                });
        });

    ev_writer.send(AppInitEvent::SetAppState(AppState::InitTileMap));
}
