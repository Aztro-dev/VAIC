use bevy::{
    app::{App, Startup, Update},
    asset::Assets,
    ecs::{
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::primitives::Plane3d,
    pbr::{AmbientLight, PbrBundle, StandardMaterial},
    prelude::{Meshable, PluginGroup},
    render::{camera::ClearColor, color::Color, mesh::Mesh, view::Msaa, view::Visibility},
    transform::components::Transform,
    utils::default,
    window::{
        Window, {PresentMode, WindowTheme},
    },
};

use bevy_framepace::*;
use bevy_infinite_grid::{
    InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin, InfiniteGridSettings,
};
use bevy_mod_raycast::{prelude::RaycastMesh, DefaultRaycastingPlugin};
// use bevy_fps_counter::FpsCounterPlugin;

mod placing;
use placing::PlacingPlugin;

mod move_objects;
use move_objects::MoveObjectsPlugin;

mod movement;
use movement::MovementPlugin;

mod ui;
use ui::UIPlugin;

mod settings;
use settings::SettingsPlugin;

mod constraints;
use constraints::ConstraintPlugin;

mod cursor;
use cursor::CursorPlugin;

mod saving;
use saving::SavingPlugin;

mod actions;
use actions::ActionsPlugin;

mod screenshot;
use screenshot::ScreenshotPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("333333").unwrap()))
        .insert_resource(Msaa::Sample4)
        .add_plugins((
            bevy::prelude::DefaultPlugins.set(bevy::window::WindowPlugin {
                primary_window: Some(Window {
                    title: "VAIC".to_string(),
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                close_when_requested: true,
                ..default()
            }),
        ))
        .add_plugins((
            PlacingPlugin,
            MoveObjectsPlugin,
            InfiniteGridPlugin,
            MovementPlugin,
            UIPlugin,
            SettingsPlugin,
            ConstraintPlugin,
            SavingPlugin,
            ActionsPlugin,
            ScreenshotPlugin,
            DefaultRaycastingPlugin,
            // FpsCounterPlugin,
            CursorPlugin,
            bevy_framepace::FramepacePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, set_frame_cap)
        .add_systems(Update, toggle_grid_visibility)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        brightness: 1000.0,
        ..default()
    });

    commands.spawn(InfiniteGridBundle {
        settings: InfiniteGridSettings {
            fadeout_distance: 500.0,
            shadow_color: None,
            ..default()
        },
        visibility: Visibility::Visible,
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(1_000_000.0, 1_000_000.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(Color::rgba(1.0, 0.0, 0.0, 0.0)),
            visibility: Visibility::Visible,
            ..default()
        },
        placing::Part {},
        RaycastMesh::<()>::default(), // Make this mesh ray cast-able;
        InfiniteGrid,
    ));
}

fn toggle_grid_visibility(
    mut visibility_query: Query<&mut Visibility, With<InfiniteGrid>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Equal) {
        let mut visibility = visibility_query.get_single_mut().unwrap();
        match *visibility {
            Visibility::Visible => {
                *visibility = Visibility::Hidden;
            }
            _ => {
                *visibility = Visibility::Visible;
            }
        }
    }
}

fn set_frame_cap(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    _loaded_settings: Res<crate::settings::Settings>,
) {
    // let limit = loaded_settings.fps_cap;
    // if limit == 0.0 {
    settings.limiter = Limiter::Off;
    // } else {
    //     settings.limiter = Limiter::from_framerate(loaded_settings.fps_cap);
    // }
}
