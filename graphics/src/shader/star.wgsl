struct CameraUniform {
    view_pos: vec4<f32>,
    target_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
    aspect: f32,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct InstanceInput {
    @location(0) direction: vec3<f32>,
    @location(1) distance: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) triangle: vec3<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    inst: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    var triangle: vec3<f32>;
    if in_vertex_index == 0 { triangle = vec3<f32>( 0.0, camera.aspect*2.0, 0.0);}
    if in_vertex_index == 1 { triangle = vec3<f32>(-1.0, -camera.aspect, 0.0);}
    if in_vertex_index == 2 { triangle = vec3<f32>( 1.0, -camera.aspect, 0.0);}

    let scale = 0.006;
    out.clip_position = vec4<f32>(triangle * scale, 1.0) + camera.view_proj * vec4<f32>(inst.direction*inst.distance, 0.0);
    out.triangle = triangle;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let brightness = max(0.5 - length(in.triangle * vec3<f32>(1.0, 1.0/camera.aspect, 1.0)), 0.0);

    let object_color = vec4<f32>(1.0, 1.0, 1.0, brightness);
    return object_color;
}