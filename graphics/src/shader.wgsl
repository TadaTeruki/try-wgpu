
struct CameraUniform {
    view_pos: vec3<f32>,
    _padding: u32,
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

struct LightUniform {
    position: vec3<f32>,
    _padding: u32,
    color: vec3<f32>,
    _padding: u32,
}

@group(2) @binding(0)
var<uniform> light: LightUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) normal: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.normal = model.normal;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let object_color: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    
    let ambient_strength = 0.05;
    let ambient_color = light.color * ambient_strength;

    let light_dir = normalize(light.position - in.clip_position.xyz);

    let diffuse_strength = max(dot(in.normal, light_dir), 0.0);
    let diffuse_color = light.color * diffuse_strength;

    let view_dir = normalize(camera.view_pos - in.clip_position.xyz);
    let half_dir = normalize(view_dir + light_dir);

    let specular_strength = pow(max(dot(in.normal, half_dir), 0.0), 32.0);
    let specular_color = light.color * specular_strength;
   
    let result = (ambient_strength + diffuse_color + specular_color) * object_color.xyz;

    return vec4<f32>(result, object_color.a);
}