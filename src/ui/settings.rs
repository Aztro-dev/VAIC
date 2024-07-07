use super::UIState;
use bevy::prelude::*;
use bevy_blur_regions::BlurRegion;
use bevy_round_ui::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UIState::Settings), spawn_ui)
            .add_systems(OnExit(UIState::Settings), despawn_ui)
            .add_systems(Update, button_system.run_if(in_state(UIState::Settings)));
    }
}

#[derive(Component)]
struct SettingsUIComponent;

const BUTTON_WIDTH: f32 = 80.0;
const BUTTON_HEIGHT: f32 = 20.0;

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<RoundUiMaterial>>,
) {
    let window_width: f32 = 800.0;
    let window_height: f32 = 800.0;
    commands
        .spawn((
            MaterialNodeBundle {
                style: Style {
                    width: Val::Percent(80.0),
                    height: Val::Percent(80.0),
                    left: Val::Percent(10.0), // 10% - 80% - 10%
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    display: Display::Flex,
                    ..default()
                },
                material: materials.add(RoundUiMaterial {
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.1),
                    border_radius: RoundUiBorder::all(crate::ui::UI_RADIUS).into(),
                    size: Vec2::new(window_width, window_height),
                    ..default()
                }),
                ..default()
            },
            SettingsUIComponent,
            BlurRegion,
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
                    background_color: BackgroundColor(Color::hex("777777").unwrap()),
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
                        top: Val::Percent(15.0),
                        bottom: Val::Percent(33.3),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::hex("777777").unwrap()),
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

fn despawn_ui(mut commands: Commands, mut ui: Query<Entity, With<SettingsUIComponent>>) {
    commands
        .entity(ui.get_single_mut().expect("Settings UI Not Found!"))
        .despawn_recursive();
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut ui_state: ResMut<NextState<UIState>>,
    mut window_query: Query<&mut Window>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut window = window_query.get_single_mut().unwrap();
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                (*window).cursor.icon = CursorIcon::Default;
                match text.sections[0].value.as_str() {
                    "Exit" => {
                        ui_state.set(UIState::Pause);
                    }
                    "Resume" => {
                        ui_state.set(UIState::Editor);
                    }
                    _ => {
                        panic!("Bruh");
                    }
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
