// The (fullscreen) input texture.
@group(0) @binding(0) 
var input_texture : texture_2d<f32>;

// The luma sampler.
//@group(0)@binding(1)
//var luma_sampler: sampler;

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
};

@vertex
fn vs_main(in: VertexInput) -> @builtin(position) vec4<f32> {
    // Generate a triangle to fill the screen.
    // The approach is based on: https://stackoverflow.com/a/59739538/4593433.
    var fullscreen_vertecies = array(
        vec4<f32>(-1.0, -1.0, 0.0, 1.0),
        vec4<f32>(3.0, -1.0, 0.0, 1.0),
        vec4<f32>(-1.0, 3.0, 0.0, 1.0)
    );

    return fullscreen_vertecies[in.vertex_index];
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let color_center = textureLoad(input_texture, vec2<i32>(pos.xy), 0).rgb;

    // Get luma at fragment center.
    let luma_center = rgbToLuma(color_center);

    // Get the luma at the four adjacent fragments.
    let luma_down = rgbToLuma();


    let gray = 0.299 * color.r + 0.587 * color.g + 0.114 * color.b;

    return vec4<f32>(gray, gray, gray, 1.0);
}

fn rgbToLuma(rgb: vec3<f32>) -> f32 {
    return sqrt(dot(rgb, vec3(0.299, 0.587, 0.114)));
}