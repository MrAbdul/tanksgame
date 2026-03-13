#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct WallMaterialParams {
    atlas_rect_px: vec4<f32>,
    health_flash: vec4<f32>,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> material: WallMaterialParams;

@group(#{MATERIAL_BIND_GROUP}) @binding(1)
var wall_atlas: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(2)
var wall_atlas_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(3)
var crack_mask: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(4)
var crack_mask_sampler: sampler;

fn atlas_uv(local_uv: vec2<f32>) -> vec2<f32> {
    let atlas_size = vec2<f32>(textureDimensions(wall_atlas));
    let rect_min = material.atlas_rect_px.xy / atlas_size;
    let rect_size = material.atlas_rect_px.zw / atlas_size;

    // If your atlas samples upside down, change this to:
    // let uv = vec2<f32>(local_uv.x, 1.0 - local_uv.y);
    let uv = local_uv;

    return rect_min + uv * rect_size;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    let sampled_uv = atlas_uv(uv);

    let base = textureSample(wall_atlas, wall_atlas_sampler, sampled_uv);

    if (base.a < 0.01) {
        discard;
    }

    let health_ratio = clamp(material.health_flash.x, 0.0, 1.0);
    let flash_amount = clamp(material.health_flash.y, 0.0, 1.0);
    let damage = 1.0 - health_ratio;

    // Crack mask should be grayscale:
    // black = no crack, white = crack
    let crack = textureSample(crack_mask, crack_mask_sampler, uv).r;

    // Reveal cracks only after some damage.
    let crack_visibility = smoothstep(0.20, 0.95, damage) * crack;

    // Dark red/brown crack color
    let crack_color = vec3<f32>(0.22, 0.05, 0.05);

    // General damage tint
    let damaged_base = mix(base.rgb, vec3<f32>(base.r, base.g * 0.45, base.b * 0.45), damage * 0.75);

    // Apply cracks on top
    let cracked = mix(damaged_base, crack_color, crack_visibility);

    // Brief bright hit flash
    let flash_color = vec3<f32>(1.0, 0.92, 0.92);
    let final_rgb = mix(cracked, flash_color, flash_amount);

    return vec4<f32>(final_rgb, base.a);
}