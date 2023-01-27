struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
};

@vertex
fn vs_main(in: VertexInput) -> @builtin(position) vec4<f32> {
    // Generate a triangle to fill the screen.
    // The approach is based on: https://stackoverflow.com/a/59739538/4593433.
    if (in.vertex_index == 0u) {
        return vec4<f32>(-1.0, -1.0, 0.0, 1.0);
    }

    if (in.vertex_index == 1u) {
        return vec4<f32>(3.0, -1.0, 0.0, 1.0);
    }

    return vec4<f32>(-1.0, 3.0, 0.0, 1.0);
}

fn rgbToLuma(rgb: vec3<f32>) -> f32 {
    return sqrt(dot(rgb, vec3(0.299, 0.587, 0.114)));
}

// The (fullscreen) input texture.
@group(0) @binding(0) 
var input_texture : texture_2d<f32>;

// The linear sampler.
@group(0) @binding(1)
var linear_sampler: sampler;

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let texel_pos = vec2<i32>(pos.xy);
    let color_center = textureLoad(input_texture, texel_pos, 0).rgb;

    // Get luma at fragment center.
    let luma_center = rgbToLuma(color_center);

    // Get the luma at the four adjacent fragments.
    //let luma_down = rgbToLuma(textureSample(input_texture, linear_sampler, pos.xy, vec2<i32>(0,-1)).rgb);



    return vec4<f32>(1.0, 0.0, 1.0, 1.0);
}

