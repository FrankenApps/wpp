@group(0) @binding(0) var input_texture : texture_2d<f32>;

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
    let dimensions = textureDimensions(input_texture);

    let color = textureLoad(input_texture, vec2<i32>(pos.xy), 0);
    let gray = 0.299 * color.r + 0.587 * color.g + 0.114 * color.b;

    return vec4<f32>(gray, gray, gray, 1.0);
}