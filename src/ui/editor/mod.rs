use super::UIState;
use bevy::prelude::*;

mod parts;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UIState::Editor), spawn_ui)
            .add_systems(OnExit(UIState::Editor), despawn_ui)
            .add_systems(
                Update,
                parts::button_system.run_if(in_state(UIState::Editor)),
            );
    }
}

#[derive(Component)]
struct EditorUIComponent;

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                                    font_size: 32.0,
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
            const BUTTON_WIDTH: f32 = 80.0;
            const BUTTON_HEIGHT: f32 = 5.0;
            for str in parts::get_parts().iter() {
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

fn despawn_ui(mut commands: Commands, mut ui: Query<Entity, With<EditorUIComponent>>) {
    commands
        .entity(ui.get_single_mut().expect("Editor UI Not Found!"))
        .despawn_recursive();
}
