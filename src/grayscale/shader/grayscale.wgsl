@group(0) @binding(0) var input_texture : texture_2d<f32>;

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
};

@vertex
fn vs_main(in: VertexInput) -> @builtin(position) vec4<f32> {
    // Generate two triangles to fill the screen.

    // Upper-left triangle.
    if(in.vertex_index == 0u) {
        return vec4<f32>(-1.0, 1.0, 0.0, 1.0);
    } else if (in.vertex_index == 1u) {
        return vec4<f32>(-1.0, -1.0, 0.0, 1.0);
    } else if (in.vertex_index == 2u) {
        return vec4<f32>(1.0, 1.0, 0.0, 1.0);
    }

    // Lower-right triangle.
    else if(in.vertex_index == 3u) {
        return vec4<f32>(1.0, -1.0, 0.0, 1.0);
    } else if (in.vertex_index == 4u) {
        return vec4<f32>(-1.0, -1.0, 0.0, 1.0);
    } else {
        return vec4<f32>(1.0, 1.0, 0.0, 1.0);
    }
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let dimensions = textureDimensions(input_texture);

    let color = textureLoad(input_texture, vec2<i32>(pos.xy), 0);
    let gray = 0.299 * color.r + 0.587 * color.g + 0.114 * color.b;

    return vec4<f32>(gray, gray, gray, 1.0);
}