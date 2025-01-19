use bevy::{prelude::*, window::PrimaryWindow};

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
        Transform::from_xyz(window.width() * 0.5, window.height() * 0.5, 0.0),
    ));
}
