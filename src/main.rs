use bevy::{prelude::*, window::WindowMode};

use boss_rush::AppPlugin;
// 16:10
const SCREEN_WIDTH: i32 = 1680;
const SCREEN_HEIGHT: i32 = 1050;
// 16:9
// pub const SCREEN_WIDTH: i32 = 1920;
// pub const SCREEN_HEIGHT: i32 = 1080;
const RESOLUTION_SCALE: f32 = 0.5;
pub const PROJECT_NAME: &str = "ASCII Wars!";

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        // mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        title: PROJECT_NAME.to_string(),
                        resolution: (
                            SCREEN_WIDTH as f32 * RESOLUTION_SCALE,
                            SCREEN_HEIGHT as f32 * RESOLUTION_SCALE,
                        )
                            .into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        ) // fallback to nearest sampling
        .add_plugins(AppPlugin)
        // .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0))) // color of the screen
        .run();
}
