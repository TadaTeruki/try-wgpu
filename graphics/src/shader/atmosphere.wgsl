
struct CameraUniform {
    view_pos: vec4<f32>,
    target_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
    aspect: f32,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct SunUniform {
    position: vec4<f32>,
    color: vec3<f32>,
    _padding: u32,
}

@group(1) @binding(0)
var<uniform> sun: SunUniform;

struct EarthUniform {
    radius: f32,
    atmosphere_radius: f32,
}

@group(2) @binding(0)
var<uniform> earth: EarthUniform;

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

fn intersection_sphere(ray_origin: vec3<f32>, ray_direction: vec3<f32>, sphere_center: vec3<f32>, sphere_radius: f32) -> mat2x3<f32> {
    let oc = ray_origin - sphere_center;
    let a = dot(ray_direction, ray_direction);
    let b = 2.0 * dot(oc, ray_direction);
    let c = dot(oc, oc) - sphere_radius * sphere_radius;
    let discriminant = b * b - 4.0 * a * c;

    if (discriminant < 0.0) {
        return mat2x3<f32>(sphere_center, sphere_center);
    } else {
        let t1 = (-b - sqrt(discriminant)) / (2.0 * a);
        let t2 = (-b + sqrt(discriminant)) / (2.0 * a);
        let p1 = ray_origin + t1 * ray_direction;
        let p2 = ray_origin + t2 * ray_direction;
        return mat2x3<f32>(p1, p2);
    }
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    let position = model.position*earth.atmosphere_radius/earth.radius;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords;
    out.normal = model.normal;
    out.model_position = model.position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let object_color: vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 0.4);
    let ray_origin = camera.view_pos.xyz;
    let ray_direction = normalize(in.model_position - ray_origin);
    let intersection_earth = intersection_sphere(ray_origin, ray_direction, vec3<f32>(0.0, 0.0, 0.0), earth.radius);
    let distance = length(intersection_earth[0] - intersection_earth[1]);

    let intersection_color = vec3<f32>(1.0, 1.0, 1.0) * distance/earth.radius/4.0;


    let result = (intersection_color) * object_color.xyz;

    return vec4<f32>(result, object_color.a);
}