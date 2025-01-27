use bevy::prelude::*;

pub mod crt;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(crt::CrtPlugin);
}
