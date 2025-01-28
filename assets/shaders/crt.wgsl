#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct CrtSettings {
    intensity: f32,         // Intensity of the scanline effect
    line_thickness: f32,    // Thickness of the scanlines
    curvature: f32,         // Amount of screen curvature
    vignette_strength: f32, // Strength of vignette effect
    aberration_offset: f32,
// #ifdef SIXTEEN_BYTE_ALIGNMENT
    _webgl2_padding: vec2<f32>
// #endif
}
@group(0) @binding(2) var<uniform> settings: CrtSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Normalize the UV coordinates
    let uv = in.uv;


    // Curvature distortion
    let uv_offset = uv - vec2<f32>(0.5);
    let curvature_distortion = uv_offset * uv_offset * settings.curvature;
    let distorted_uv = uv + curvature_distortion;

    // Sample the base color with curvature distortion
    let color = textureSample(screen_texture, texture_sampler, distorted_uv);

    // Apply scanline effect
    let line_factor = sin(in.position.y * settings.line_thickness * 3.141592) * 0.5 + 0.5;
    let scanline_color = mix(color.rgb * (1.0 - settings.intensity), color.rgb, line_factor);

    // Color aberration (RGB channel shift)
    let red = textureSample(screen_texture, texture_sampler, distorted_uv + vec2<f32>(settings.aberration_offset, 0.0));
    let green = textureSample(screen_texture, texture_sampler, distorted_uv);
    let blue = textureSample(screen_texture, texture_sampler, distorted_uv - vec2<f32>(settings.aberration_offset, 0.0));
    let aberrated_color = vec3<f32>(red.r, green.g, blue.b);

    // Vignette effect
    let vignette = smoothstep(0.7, 0.3, length(uv_offset)) * settings.vignette_strength;

    // Combine effects
    let final_color = mix(scanline_color, aberrated_color, vignette);

    return vec4<f32>(final_color, color.a);




}
