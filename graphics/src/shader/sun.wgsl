struct CameraUniform {
    view_pos: vec4<f32>,
    target_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
    aspect: f32,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) triangle: vec3<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    input: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    var triangle: vec3<f32>;
    if in_vertex_index == 0 { triangle = vec3<f32>( 0.0, camera.aspect*2.0, 0.0);}
    if in_vertex_index == 1 { triangle = vec3<f32>(-1.0, -camera.aspect, 0.0);}
    if in_vertex_index == 2 { triangle = vec3<f32>( 1.0, -camera.aspect, 0.0);}

    let scale = 0.1;
    out.clip_position = vec4<f32>(triangle * scale, 1.0) + camera.view_proj * vec4<f32>(normalize(input.position), 0.0);
    out.triangle = triangle;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let potential = max(0.5 - length(in.triangle * vec3<f32>(1.0, 1.0/camera.aspect, 1.0)), 0.0)*2.0;
    let potential_inner = 0.9;
    let brightness = pow(min(potential/potential_inner, 1.0), 16.0);

    let object_color = vec4<f32>(1.0, 1.0, 0.98, brightness);
    return object_color;
}