use super::PickerSelect;
use super::PickerState;
use bevy::a11y::{
    accesskit::{NodeBuilder, Role},
    AccessibilityNode,
};
use bevy::input::mouse::*;
use bevy::prelude::*;

pub fn spawn_part_picker(mut commands: Commands) {
    let parts = vec![
        "Cube", "Sphere", "Cone", "Cube", "Sphere", "Cone", "Cube", "Sphere", "Cone", "Cube",
        "Sphere", "Cone", "Cube", "Sphere", "Cone",
    ];
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                left: Val::Percent(83.0), // Magic number
                width: Val::Percent(40.0),
                height: Val::Percent(80.0),
                top: Val::Percent(10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Part Picker
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(40.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::hex("4A4A4AAA").unwrap()),
                    ..default()
                })
                .with_children(|parent| {
                    // List of parts
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::End,
                                    justify_items: JustifyItems::Center,
                                    overflow: Overflow::clip_y(),
                                    width: Val::Percent(100.0),
                                    ..default()
                                },
                                ..default()
                            },
                            ScrollingList::default(),
                        ))
                        .with_children(|parent| {
                            // Each individual part
                            for part in parts {
                                part_button_styled(parent, part);
                            }
                        });
                });
            // Collapse button
        });
}

pub fn detect_button_click(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    text_query: Query<&mut Text>,
    mut next: ResMut<NextState<PickerState>>,
    mut picker_select: ResMut<PickerSelect>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                let text_val = &text.sections[0].value;
                *color = Color::hex("202020").unwrap().into();

                if picker_select.selected == None {
                    next.set(PickerState::PartSelected);
                    picker_select.selected = Some(text_val.to_string());
                } else {
                    next.set(PickerState::PartDeselected);
                    picker_select.selected = None;
                }
            }
            Interaction::Hovered => {
                *color = Color::hex("2A2A2A").unwrap().into();
            }
            Interaction::None => {
                *color = Color::hex("3A3A3A").unwrap().into();
            }
        }
    }
}

fn part_button_styled(parent: &mut ChildBuilder, text: &str) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(2.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::End,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::hex("4A4A4A").unwrap()),
                border_color: BorderColor(Color::hex("303030").unwrap()),
                ..default()
            },
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 20.0,
                        ..default()
                    },
                ),
                Label,
            ));
        });
}

#[derive(Component, Default)]
pub struct ScrollingList {
    position: f32,
}

pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}
