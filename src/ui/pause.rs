use super::UIState;
use bevy::app::AppExit;
use bevy::prelude::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UIState::Pause), spawn_ui)
            .add_systems(OnExit(UIState::Pause), despawn_ui)
            .add_systems(Update, button_system.run_if(in_state(UIState::Pause)));
    }
}

#[derive(Component)]
struct PauseUIComponent;

const BUTTON_WIDTH: f32 = 80.0;
const BUTTON_HEIGHT: f32 = 20.0;

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(40.0),
                    height: Val::Percent(80.0),
                    left: Val::Percent(30.0), // 30% - 40% - 30%
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(Color::hex("444444").unwrap().into()),
                ..default()
            },
            PauseUIComponent,
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Percent(BUTTON_WIDTH),
                        height: Val::Percent(BUTTON_HEIGHT),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::hex("777777").unwrap().into()),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Resume",
                            TextStyle {
                                font: asset_server.load("FiraMonoNerdFontMono-Bold.otf"),
                                font_size: 32.0,
                                ..default()
                            },
                        ),
                        Label,
                    ));
                });
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Percent(BUTTON_WIDTH),
                        height: Val::Percent(BUTTON_HEIGHT),
                        top: Val::Percent(10.0),
                        bottom: Val::Percent(33.3),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::hex("777777").unwrap().into()),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Exit",
                            TextStyle {
                                font: asset_server.load("FiraMonoNerdFontMono-Bold.otf"),
                                font_size: 32.0,
                                ..default()
                            },
                        ),
                        Label,
                    ));
                });
        });
}

fn despawn_ui(mut commands: Commands, mut ui: Query<Entity, With<PauseUIComponent>>) {
    commands
        .entity(ui.get_single_mut().expect("Pause UI Not Found!"))
        .despawn_recursive();
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut exit: EventWriter<AppExit>,
    mut ui_state: ResMut<NextState<UIState>>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                match text.sections[0].value.as_str() {
                    "Exit" => {
                        exit.send(AppExit);
                    }
                    "Resume" => {
                        ui_state.set(UIState::None);
                    }
                    _ => {
                        panic!("Bruh");
                    }
                }
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
