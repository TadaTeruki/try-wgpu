
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
    let a = dot(ray_direction, ray_direction)+0.000001;
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
    out.clip_position = camera.view_proj * vec4<f32>(position, 1.0);
    out.tex_coords = model.tex_coords;
    out.normal = model.normal;
    out.model_position = position;
    return out;
}

fn vector3_equals(a: vec3<f32>, b: vec3<f32>) -> bool {
    return a.x == b.x && a.y == b.y && a.z == b.z;
}

const DENSITY_FALLOFF: f32 = 3.0;

fn atmosphere_density(position: vec3<f32>, center: vec3<f32>) -> f32 {
    let distance = length(position - center);
    let t = min(max((distance-earth.radius)/(earth.atmosphere_radius-earth.radius), 0.0), 1.0);
    return exp(-t*DENSITY_FALLOFF)*(1-t);
}

fn optical_depth(ray_start: vec3<f32>, ray_end: vec3<f32>, center: vec3<f32>) -> f32 {
    let division = 5;
    let sample_interval = (ray_end - ray_start) / f32(division-1);
    var optical_depth_sum = 0.0;
    for (var i: i32 = 0; i < division; i++) {
        let sample_position = ray_start + sample_interval * f32(i);
        let local_density = atmosphere_density(sample_position, center);
        optical_depth_sum = optical_depth_sum + local_density;
    }
    return optical_depth_sum/f32(division);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let ray_origin = camera.view_pos.xyz;
    let ray_direction = normalize(in.model_position - ray_origin);
    let center = vec3<f32>(0.0, 0.0, 0.0);
    let intersection_ray_earth = intersection_sphere(ray_origin, ray_direction, center, earth.radius);
    let intersection_ray_atmosphere = intersection_sphere(ray_origin, ray_direction, center, earth.atmosphere_radius);
    var atmosphere_start: vec3<f32> = intersection_ray_atmosphere[0];
    var atmosphere_end: vec3<f32> = intersection_ray_earth[0];
    if (vector3_equals(atmosphere_end, center)) {
        atmosphere_end = intersection_ray_atmosphere[1];
    }
    
    var strength_average: f32 = 0.0;
    if (!vector3_equals(atmosphere_start, center) && !vector3_equals(atmosphere_end, center)) {
        let division: i32 = 20;
        var strength_sum: f32 = 0.0;
        let sample_interval = (atmosphere_end - atmosphere_start) / f32(division-1);
        for (var i: i32 = 0; i < division; i++) {
            let sample_position = atmosphere_start + sample_interval * f32(i);
            
            let sample_sun_dir = normalize(sample_position-sun.position.xyz);
            
            let intersection_sun_atmosphere = intersection_sphere(sun.position.xyz, sample_sun_dir, center, earth.atmosphere_radius);
            
            // this is not correct
            let sun_dir = normalize(sun.position.xyz - sample_position);
            var diffuse_strength = min(max(-dot(-sun_dir, in.normal), 0.0), 1.0);
            
            let sun_ray_optical_depth = optical_depth(intersection_sun_atmosphere[0], sample_position, center);
            let transmittance = exp(-sun_ray_optical_depth);

            let local_density = atmosphere_density(sample_position, center);
            strength_sum = strength_sum + transmittance * local_density * diffuse_strength;
        }

        strength_average = strength_sum/f32(division);
    }

    let object_color: vec4<f32> = vec4<f32>(sun.color, strength_average*0.3);

    return object_color;
}
