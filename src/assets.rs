use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, load_atlas);
}

fn load_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Setup the sprite sheet
    let texture_handle = asset_server.load("terminal8x8_transparent.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(8 as u32, 8 as u32), 16, 16, None, None);
    let layout_handle = atlases.add(layout);

    let charset = CharsetAsset {
        atlas: layout_handle.clone(),
        texture: texture_handle.clone(),
    };
    // add sprite atlas as resource
    commands.insert_resource(charset.clone());
}
