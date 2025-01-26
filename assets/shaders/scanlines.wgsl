// This shader computes a scanline effect.

// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct PostProcessSettings {
    intensity: f32, // Intensity of the scanline effect
    line_thickness: f32, // Thickness of the scanlines
#ifdef SIXTEEN_BYTE_ALIGNMENT
    _webgl2_padding: vec2<f32>
#endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Sample the base color from the screen texture
    let color = textureSample(screen_texture, texture_sampler, in.uv);

    // Determine the scanline pattern
    let line_factor = sin(in.position.y * settings.line_thickness * 3.141592) * 0.5 + 0.5;

    // Apply the scanline effect by blending the color based on the line factor and intensity
    let scanline_color = mix(color.rgb * (1.0 - settings.intensity), color.rgb, line_factor);

    return vec4<f32>(scanline_color, color.a);
}



// struct VertexOutput {
//     @builtin(position) Position: vec4<f32>,
//     @location(0) TexCoords: vec2<f32>,
// };

// @fragment
// fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
//     let tex_color = textureLoad(texture, in.TexCoords);

//     // Calculate scanline position
//     let scanline_pos = fract(in.TexCoords.y * 10.0); 

//     // Create a scanline mask
//     let scanline_mask = step(0.1, scanline_pos) * step(scanline_pos, 0.9); 

//     // Apply scanline mask to color
//     return tex_color * vec4<f32>(scanline_mask);
// }