use bevy::prelude::*;

pub mod example;
pub mod scanlines;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((scanlines::ScanlinePlugin));
}
