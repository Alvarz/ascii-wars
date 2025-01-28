use bevy::{core_pipeline::post_process::ChromaticAberration, prelude::*, window::PrimaryWindow};

use crate::postprocessing::crt::CrtSettings;

#[derive(Component)]
pub struct MainCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_camera);
}

fn initialize_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        MainCamera,
        Camera2d {},
        CrtSettings {
            line_thickness: 0.40,
            intensity: 0.5,
            curvature: 0.1,
            aberration_offset: 0.0,
            vignette_strength: 50.0,
            ..default()
        },
        ChromaticAberration {
            intensity: 0.009,
            ..default()
        },
        Transform::from_xyz(window.width() * 0.5, window.height() * 0.5, 0.0),
    ));
}
