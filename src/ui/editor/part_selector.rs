use crate::placing::PlacingEvent;
use crate::ui::editor::part_selector;
use crate::ui::editor::EditorUIComponent;
use bevy::prelude::*;

pub fn spawn_part_selector(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(15.0),
                    height: Val::Percent(80.0),
                    left: Val::Percent(80.0), // 80% - 15% - 5%
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    display: Display::Flex,
                    ..default()
                },
                background_color: BackgroundColor(Color::hex("444444").unwrap()),
                ..default()
            },
            EditorUIComponent,
        ))
        .with_children(|parent| {
            // Title "Parts"
            parent
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
                                "Parts",
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
            // End "Parts"
            // Parts List
            const BUTTON_WIDTH: f32 = 100.0;
            const BUTTON_HEIGHT: f32 = 5.0;
            for str in part_selector::get_parts().iter() {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Percent(BUTTON_WIDTH),
                            height: Val::Percent(BUTTON_HEIGHT),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::bottom(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::hex("777777").unwrap()),
                        border_color: BorderColor(Color::hex("555555").unwrap()),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                str,
                                TextStyle {
                                    font: asset_server.load("FiraMonoNerdFontMono-Bold.otf"),
                                    font_size: 24.0,
                                    ..default()
                                },
                            ),
                            Label,
                        ));
                    });
            }
        });
}

pub fn get_parts() -> Vec<String> {
    return vec![
        String::from("Duck"),
        String::from("Cube"),
        String::from("Penguin"),
    ];
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut placing_event: EventWriter<PlacingEvent>,
    model_handles: Res<crate::ui::editor::handle::ModelHandles>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                let name = get_model_name(text.sections[0].value.as_str());

                let formatted = format!("models/{name}#Scene0");

                let model_handle = crate::ui::editor::handle::get_model_handle(
                    formatted.clone(),
                    (*model_handles).clone(),
                );

                placing_event.send(PlacingEvent(formatted.clone(), model_handle.clone()));
                *color = Color::hex("AAAAAA").unwrap().into();
            }
            Interaction::Hovered => {
                *color = Color::hex("999999").unwrap().into();
            }
            Interaction::None => {
                *color = Color::hex("777777").unwrap().into();
            }
        }
    }
}

pub fn get_model_name(text: &str) -> &str {
    match text {
        "Duck" => "low_poly_duck.glb",
        "Cube" => "cube.glb",
        "Penguin" => "penguin.glb",
        _ => "duck",
    }
}

pub fn reverse_model_name(text: String) -> String {
    match text.as_str() {
        "models/low_poly_duck.glb#Scene0" => String::from("Duck"),
        "models/cube.glb#Scene0" => String::from("Cube"),
        "models/penguin.glb#Scene0" => String::from("Penguin"),
        _ => {
            panic!();
        }
    }
}
