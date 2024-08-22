struct CameraUniform {
    view_pos: vec3<f32>,
    _padding: u32,
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    if in_vertex_index == 0 { out.clip_position = vec4<f32>(-1, -1, 0, 1);}
    if in_vertex_index == 1 { out.clip_position = vec4<f32>( 3, -1, 0, 1);}
    if in_vertex_index == 2 { out.clip_position = vec4<f32>(-1,  3, 0, 1);}
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let y = in.clip_position.y;
    let brightness = 1.0 - (y + 1.0) / 2.0;
    let object_color = vec4<f32>(brightness, brightness, brightness, 0.0);
    return object_color;
}