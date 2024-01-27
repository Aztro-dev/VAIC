use bevy::core::TaskPoolThreadAssignmentPolicy;
use bevy::prelude::*;
use bevy::tasks::available_parallelism;
use bevy::window::{PresentMode, WindowTheme};
use bevy_fps_counter::FpsCounterPlugin;
use bevy_framepace::*;
use bevy_infinite_grid::{
    InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin, InfiniteGridSettings,
};
use bevy_mod_raycast::prelude::*;

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

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("333333").unwrap()))
        .insert_resource(Msaa::Sample4)
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "VAIC".to_string(),
                        fit_canvas_to_parent: true,
                        present_mode: PresentMode::AutoVsync,
                        window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    close_when_requested: true,
                    ..default()
                })
                .set(TaskPoolPlugin {
                    task_pool_options: TaskPoolOptions {
                        compute: TaskPoolThreadAssignmentPolicy {
                            // set the minimum # of compute threads
                            // to the total number of available threads
                            min_threads: available_parallelism(),
                            max_threads: std::usize::MAX, // unlimited max threads
                            percent: 1.0,                 // this value is irrelevant in this case
                        },
                        // keep the defaults for everything else
                        ..default()
                    },
                }),
            PlacingPlugin,
            MoveObjectsPlugin,
            InfiniteGridPlugin,
            MovementPlugin,
            UIPlugin,
            SettingsPlugin,
            ConstraintPlugin,
            SavingPlugin,
            DefaultRaycastingPlugin,
            FpsCounterPlugin,
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
        color: Color::WHITE,
        brightness: 3.0,
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
            mesh: meshes.add(
                shape::Plane {
                    size: 1_000_000.0, // Basically infinite if you really think about it
                    ..default()
                }
                .into(),
            ),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(Color::rgba(1.0, 0.0, 0.0, 0.0).into()),
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
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Equals) {
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
