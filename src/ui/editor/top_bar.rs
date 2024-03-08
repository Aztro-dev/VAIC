use crate::saving::{save_timer, SaveIcon};
use crate::ui::editor::EditorUIComponent;
use bevy::prelude::*;

#[derive(Component)]
pub struct EditorTopBar;

#[derive(Component)]
pub struct EditorTopBarSaveTimer;

pub fn spawn_top_bar(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    last_save: Res<save_timer::MostRecentSave>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(5.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::hex("444444").unwrap()),
                ..default()
            },
            EditorUIComponent,
            EditorTopBar,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        ..default()
                    },
                    text: Text::from_section(
                        last_save.to_string(),
                        TextStyle {
                            font: asset_server.load("FiraMonoNerdFontMono-Bold.otf"),
                            font_size: 8.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                },
                EditorTopBarSaveTimer,
            ));

            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: asset_server.load("images/save_icon.png"),
                    ..default()
                },
                style: Style {
                    height: Val::Percent(90.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                visibility: Visibility::Hidden,
                ..default()
            });

            parent
                .spawn(ImageBundle {
                    image: UiImage {
                        texture: asset_server.load("images/save_icon.png"),
                        ..default()
                    },
                    style: Style {
                        height: Val::Percent(90.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    visibility: Visibility::Hidden,
                    ..default()
                })
                .insert(SaveIcon);
        });
}

pub fn update_top_bar_timer(
    mut top_bar: Query<&mut Text, With<EditorTopBarSaveTimer>>,
    update_timer: ResMut<save_timer::UpdateSaveCountTimer>,
    last_save: Res<save_timer::MostRecentSave>,
    asset_server: Res<AssetServer>,
) {
    if !update_timer.timer.finished() {
        return;
    }
    for mut text in top_bar.iter_mut() {
        *text = Text::from_section(
            last_save.to_string(),
            TextStyle {
                font: asset_server.load("FiraMonoNerdFontMono-Bold.otf"),
                font_size: 16.0,
                color: Color::WHITE,
            },
        );
    }
}
