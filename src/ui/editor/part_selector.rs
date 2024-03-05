use crate::constraints::ConstrainState;
use crate::placing::PlacingEvent;
use crate::ui::editor::part_selector;
use crate::ui::editor::EditorUIComponent;
use bevy::prelude::*;
use bevy_round_ui::prelude::*;

pub fn spawn_part_selector(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<RoundUiMaterial>>,
) {
    let window_width: f32 = 150.0;
    let window_height: f32 = 800.0;
    commands
        .spawn((
            MaterialNodeBundle {
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
                material: materials.add(RoundUiMaterial {
                    background_color: Color::hex("444444").unwrap(),
                    border_radius: RoundUiBorder::all(crate::ui::UI_RADIUS).into(),
                    size: Vec2::new(window_width, window_height),
                    ..default()
                }),
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
        String::from("2x25 C-Channel"),
        String::from("2x1 C-Channel"),
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
    constrain_state: Res<State<ConstrainState>>,
    mut window_query: Query<&mut Window>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut window = window_query.get_single_mut().unwrap();
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                if *constrain_state != ConstrainState::Constraining {
                    let name = get_model_name(text.sections[0].value.as_str());

                    let formatted = format!("models/{name}#Scene0");

                    let model_handle = crate::ui::editor::handle::get_model_handle(
                        formatted.clone(),
                        (*model_handles).clone(),
                    );

                    placing_event.send(PlacingEvent(formatted.clone(), model_handle.clone()));
                }
                *color = Color::hex("AAAAAA").unwrap().into();
            }
            Interaction::Hovered => {
                (*window).cursor.icon = CursorIcon::Pointer;
                *color = Color::hex("999999").unwrap().into();
            }
            Interaction::None => {
                (*window).cursor.icon = CursorIcon::Default;
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
        "2x25 C-Channel" => "c-channel-1x2x1x25.glb",
        "2x1 C-Channel" => "c-channel-1x2x1x1.glb",
        _ => "duck",
    }
}

pub fn reverse_model_name(text: String) -> String {
    match text.as_str() {
        "models/low_poly_duck.glb#Scene0" => String::from("Duck"),
        "models/cube.glb#Scene0" => String::from("Cube"),
        "models/penguin.glb#Scene0" => String::from("Penguin"),
        "models/c-channel-1x2x1x25.glb#Scene0" => String::from("2x25 C-Channel"),
        "models/c-channel-1x2x1x1.glb#Scene0" => String::from("2x1 C-Channel"),
        _ => {
            panic!("{}", text.as_str());
        }
    }
}
