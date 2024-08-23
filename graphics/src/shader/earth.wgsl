
struct CameraUniform {
    view_pos: vec4<f32>,
    target_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
    aspect: f32,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

struct SunUniform {
    position: vec4<f32>,
    color: vec3<f32>,
    _padding: u32,
}

@group(2) @binding(0)
var<uniform> sun: SunUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) model_position: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords;
    out.normal = model.normal;
    out.model_position = model.position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let object_color: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    
    let ambient_strength = 0.0;
    let ambient_color = sun.color * ambient_strength;

    let sun_dir = normalize(sun.position.xyz - in.model_position);

    let diffuse_strength = min(max(dot(sun_dir, in.normal), 0.0), 1.0);
    let diffuse_color = sun.color * diffuse_strength;

    let view_dir = normalize(camera.view_pos.xyz - in.model_position);
    let reflect_dir = reflect(-sun_dir, in.normal);

    let specular_strength = pow(max(dot(reflect_dir, view_dir), 0.0), 18.0);
    let specular_color = sun.color * specular_strength;
   
    let result = (ambient_strength + diffuse_color + specular_color) * object_color.xyz;

    return vec4<f32>(result, object_color.a);
}