use crate::saving::save_timer;
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
                    justify_content: JustifyContent::Start,
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
            let most_recent_save = last_save.time_since_last_save();
            let mut most_recent_save_str = "Not saved yet!".to_string();
            if most_recent_save.is_some() {
                let most_recent_save = most_recent_save.unwrap();
                if most_recent_save == 1 {
                    most_recent_save_str = "Saved 1 sec ago".to_string();
                } else if most_recent_save < 60 {
                    most_recent_save_str = format!("Saved {} secs ago", most_recent_save);
                } else if most_recent_save >= 60 && most_recent_save < 120 {
                    most_recent_save_str = "Saved 1 min ago".to_string();
                } else {
                    most_recent_save_str = format!("Saved {} mins ago", most_recent_save / 60);
                }
            }
            parent.spawn((
                TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    text: Text::from_section(
                        most_recent_save_str,
                        TextStyle {
                            font: asset_server.load("FiraMonoNerdFontMono-Bold.otf"),
                            font_size: 12.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                },
                EditorTopBarSaveTimer,
            ));
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
        let most_recent_save = last_save.time_since_last_save();
        let mut most_recent_save_str = "Not saved yet!".to_string();
        if most_recent_save.is_some() {
            let most_recent_save = most_recent_save.unwrap();
            if most_recent_save == 1 {
                most_recent_save_str = "Saved 1 sec ago".to_string();
            } else if most_recent_save < 60 {
                most_recent_save_str = format!("Saved {} secs ago", most_recent_save);
            } else if most_recent_save >= 60 && most_recent_save < 120 {
                most_recent_save_str = "Saved 1 min ago".to_string();
            } else {
                most_recent_save_str = format!("Saved {} mins ago", most_recent_save / 60);
            }
        }
        *text = Text::from_section(
            most_recent_save_str,
            TextStyle {
                font: asset_server.load("FiraMonoNerdFontMono-Bold.otf"),
                font_size: 12.0,
                color: Color::WHITE,
            },
        );
    }
}
