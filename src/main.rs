use bevy::prelude::*;

use boss_rush::AppPlugin;

pub const SCREEN_WIDTH: i32 = 142;
pub const SCREEN_HEIGHT: i32 = 80;
// pub const UI_HEIGHT: i32 = 10;
pub const PROJECT_NAME: &str = "ASCII Wars!";

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: PROJECT_NAME.to_string(),
                        resolution: (SCREEN_WIDTH as f32 * 10.0, SCREEN_HEIGHT as f32 * 10.0)
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
