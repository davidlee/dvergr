use bevy::prelude::*;
use bevy_pancam::PanCam;

// use std::fmt::Alignment;

pub fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());

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
            background_color: BackgroundColor(Color::DARK_GREEN),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                // top full width menu bar
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Px(50.),
                        // for contained text
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgb(0.1, 0.1, 0.15)),
                    // border_color: Color::RED.into(),
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
                            background_color: BackgroundColor(Color::rgb(0.05, 0.0, 0.0)),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("Sidebar", heading_style.clone()));
                        });
                    // Main Content Area
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                // width: Val::Percent(100.),
                                flex_grow: 100.,
                                height: Val::Percent(100.),
                                // for contained text
                                // align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::rgb(0.05, 0.05, 0.10)),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // parent.spawn(TextBundle::from_section(
                            //     "Main Content",
                            //     heading_style.clone(),
                            // ));
                            parent
                                .spawn(NodeBundle {
                                    /// Map Area
                                    style: Style {
                                        // width: Val::Percent(100.),
                                        flex_grow: 100.,
                                        height: Val::Percent(70.),
                                        // for contained text
                                        // align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::rgb(0.05, 0.05, 0.10)),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Map Area",
                                        text_style.clone(),
                                    ));
                                });

                            parent
                                .spawn(NodeBundle {
                                    // Console
                                    style: Style {
                                        // width: Val::Percent(100.),
                                        flex_grow: 100.,
                                        height: Val::Percent(30.),
                                        // for contained text
                                        // align_items: AlignItems::Center,
                                        // justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::rgb(0.0, 0.0, 0.0)),
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
                        // for contained text
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::rgb(0.1, 0.1, 0.15)),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Footer", heading_style.clone()));
                });
        });
}
