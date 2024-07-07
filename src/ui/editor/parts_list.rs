use crate::{actions::ActionList, placing::PlacedPart, ui::editor::EditorUIComponent};
use bevy::prelude::*;
use bevy_blur_regions::BlurRegion;

#[derive(Component)]
pub struct PartsList;

pub fn spawn_parts_list(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    placed_already: Res<ActionList>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(15.0),
                    height: Val::Percent(80.0),
                    left: Val::Percent(0.0), // 80% - 15% - 5%
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    display: Display::Flex,
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.1)),
                ..default()
            },
            EditorUIComponent,
            PartsList,
            BlurRegion,
        ))
        .with_children(|parent| {
            // Parts header
            parent
                .spawn(NodeBundle {
                    background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.1)),
                    style: Style {
                        width: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        display: Display::Flex,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(
                                "Parts List",
                                TextStyle {
                                    font: asset_server.load("FiraMonoNerdFontMono-Bold.otf"),
                                    font_size: 24.0,
                                    ..default()
                                },
                            ),
                            ..default()
                        },
                        Label,
                    ));
                });
            // End Parts header
            // Show already placed parts
            for part in placed_already.0.iter() {
                if part.is_placeholder() {
                    continue;
                }
                if !part.is_placed() {
                    return;
                }
                parent
                    .spawn(NodeBundle {
                        background_color: BackgroundColor(Color::hex("777777").unwrap()),
                        style: Style {
                            width: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            overflow: Overflow::clip_y(),
                            display: Display::Flex,
                            border: UiRect::bottom(Val::Px(1.0)),
                            ..default()
                        },
                        border_color: BorderColor(Color::hex("555555").unwrap()),
                        ..default()
                    })
                    .with_children(|parent| {
                        let part: PlacedPart = part.clone().into();
                        parent.spawn((
                            TextBundle {
                                text: Text::from_section(
                                    crate::ui::editor::part_selector::reverse_model_name(
                                        part.name.clone(),
                                    ),
                                    TextStyle {
                                        font: asset_server.load("FiraMonoNerdFontMono-Bold.otf"),
                                        font_size: 24.0,
                                        ..default()
                                    },
                                ),
                                ..default()
                            },
                            Label,
                        ));
                    });
            }
        });
}

pub fn update_parts_list(
    mut commands: Commands,
    mut parts_list_query: Query<Entity, With<PartsList>>,
    recently_placed: Res<ActionList>,
    asset_server: Res<AssetServer>,
) {
    if !recently_placed.is_changed() {
        return;
    }
    if recently_placed.0.is_empty() {
        return;
    }
    if !recently_placed.0[recently_placed.0.len() - 1].is_placed() {
        return;
    }
    let parts_list = parts_list_query.get_single_mut().unwrap();

    let new_part = NodeBundle {
        background_color: BackgroundColor(Color::hex("777777").unwrap()),
        style: Style {
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            border: UiRect::bottom(Val::Px(1.0)),
            ..default()
        },
        border_color: BorderColor(Color::hex("555555").unwrap()),
        ..default()
    };

    let part: PlacedPart = recently_placed.0[recently_placed.0.len() - 1]
        .clone()
        .into();

    let recently_placed_stripped =
        crate::ui::editor::part_selector::reverse_model_name(part.name.clone());
    let new_part_text = (
        TextBundle {
            text: Text::from_section(
                recently_placed_stripped,
                TextStyle {
                    font: asset_server.load("FiraMonoNerdFontMono-Bold.otf"),
                    font_size: 24.0,
                    ..default()
                },
            ),
            ..default()
        },
        Label,
    );

    let new_part_entity = commands.spawn(new_part).id();
    let new_part_text_entity = commands.spawn(new_part_text).id();

    commands
        .entity(new_part_entity)
        .add_child(new_part_text_entity);

    commands.entity(parts_list).add_child(new_part_entity);
}

#[derive(Event)]
pub struct RefreshPartsList;

pub fn refresh_parts_list(
    mut commands: Commands,
    mut parts_list_query: Query<Entity, With<PartsList>>,
    recently_placed: Res<ActionList>,
    asset_server: Res<AssetServer>,
    mut refresh_parts_list_reader: EventReader<RefreshPartsList>,
) {
    for _event in refresh_parts_list_reader.read() {
        let parts_list = parts_list_query.get_single_mut().unwrap();
        commands.entity(parts_list).despawn_descendants();
        let header = commands
            .spawn(NodeBundle {
                background_color: BackgroundColor(Color::hex("666666").unwrap()),
                style: Style {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    display: Display::Flex,
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    TextBundle {
                        text: Text::from_section(
                            "Parts List",
                            TextStyle {
                                font: asset_server.load("FiraMonoNerdFontMono-Bold.otf"),
                                font_size: 24.0,
                                ..default()
                            },
                        ),
                        ..default()
                    },
                    Label,
                ));
            })
            .id();

        commands.entity(parts_list).add_child(header);
        for action in recently_placed.0.iter() {
            if !action.is_placed() {
                continue;
            }
            let new_part = NodeBundle {
                background_color: BackgroundColor(Color::hex("777777").unwrap()),
                style: Style {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    display: Display::Flex,
                    border: UiRect::bottom(Val::Px(1.0)),
                    ..default()
                },
                border_color: BorderColor(Color::hex("555555").unwrap()),
                ..default()
            };

            let placed: PlacedPart = action.clone().into();

            let recently_placed_stripped =
                crate::ui::editor::part_selector::reverse_model_name(placed.name.clone());
            let new_part_text = (
                TextBundle {
                    text: Text::from_section(
                        recently_placed_stripped,
                        TextStyle {
                            font: asset_server.load("FiraMonoNerdFontMono-Bold.otf"),
                            font_size: 24.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                Label,
            );

            let new_part_entity = commands.spawn(new_part).id();
            let new_part_text_entity = commands.spawn(new_part_text).id();

            commands
                .entity(new_part_entity)
                .add_child(new_part_text_entity);

            commands.entity(parts_list).add_child(new_part_entity);
        }
    }
}
