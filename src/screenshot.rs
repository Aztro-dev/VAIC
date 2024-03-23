use bevy::prelude::*;
use bevy::render::view::screenshot::ScreenshotManager;
use bevy::window::PrimaryWindow;
use bevy_infinite_grid::InfiniteGrid;
pub struct ScreenshotPlugin;

impl Plugin for ScreenshotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, screenshot_system);
    }
}

fn screenshot_system(
    input: Res<ButtonInput<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
    mut ui_query: Query<&mut Visibility, (With<Node>, Without<InfiniteGrid>)>,
    mut grid_query: Query<&mut Visibility, (With<InfiniteGrid>, Without<Node>)>,
) {
    if input.just_pressed(KeyCode::F12) {
        for mut visibility in grid_query.iter_mut() {
            *visibility = Visibility::Hidden;
        }
        let mut visibility_query_original =
            ui_query.iter_mut().map(|v| *v).collect::<Vec<Visibility>>();
        for mut visibility in ui_query.iter_mut() {
            *visibility = Visibility::Hidden;
        }
        let path = format!("./screenshot-{}.png", *counter);
        *counter += 1;
        screenshot_manager
            .save_screenshot_to_disk(main_window.single(), path)
            .unwrap();
        for visibility in visibility_query_original.iter_mut() {
            *visibility = Visibility::Visible;
        }
        for mut visibility in grid_query.iter_mut() {
            *visibility = Visibility::Visible;
        }
    }
}
