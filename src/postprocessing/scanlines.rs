use bevy::prelude::*;

// #[derive(Default, Clone, Copy, Shader)]
// pub struct ScanlineShader {
//     #[from(wgpu)]
//     shader: ShaderStages<VS = (), FS = wgsl::Shader>,
// }

// const SCANLINE_SHADER_HANDLE: Handle<ScanlineShader> = Handle::weak();

// fn scanline_material_system(mut commands: Commands, mut materials: ResMut<Assets<ScanlineShader>>) {
//     if materials.get(&SCANLINE_SHADER_HANDLE).is_none() {
//         commands.insert_resource(SCANLINE_SHADER_HANDLE.typed());
//     }
// }

// fn add_scanline_post_process(
//     mut commands: Commands,
//     mut materials: ResMut<Assets<ScanlineShader>>,
// ) {
//     commands.insert_resource(PostProcessBundle {
//         render_graph: bevy::core_pipeline::core_3d::graph::PostProcessGraph::default(),
//         post_process: PostProcess {
//             systems: vec![SystemStage::single_threaded().with_system(scanline_material_system)],
//             ..default()
//         },
//     });
// }

pub(super) fn plugin(app: &mut App) {
    //     app.add_systems(Startup, (add_scanline_post_process,));
    //     app.add_systems(Startup, setup)
}
