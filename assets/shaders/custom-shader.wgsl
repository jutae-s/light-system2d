@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> u_transform: mat4x4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> u_view_proj: mat4x4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> u_circle_pos: vec3<f32>;


//vec normalization that avoids division by zero.
fn safe_norm(v: vec3<f32>) -> vec3<f32> {
    let len = length(v);
    return select(vec3<f32>(0.0), v / len, len > 1e-5);
}
// vertex output with only clipspace positions
struct VOut { @builtin(position) clip_pos: vec4<f32> };

@vertex
fn vertex_main(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) edge_n: vec2<f32>,
    @location(4) edge_m: vec2<f32>,
    @location(5) end_flag: u32,
) -> VOut {
    var out: VOut;
    //vertex into world space
    var world_p = (u_transform * vec4<f32>(position, 1.0)).xyz;

    //edge data into world space
    let nW = normalize((u_transform * vec4<f32>(edge_n.x, edge_n.y, 0.0, 0.0)).xyz);
    let mW = (u_transform * vec4<f32>(edge_m.x, edge_m.y, 0.0, 1.0)).xyz;

    //light distance from edge midpoint
    let Ledge = safe_norm(mW - u_circle_pos);

    //if the light is close to the edge and facing away, push it out to avoid shadow acne.
    let eps: f32 = 0.02;
    let edge_shadow = dot(nW, Ledge) > eps;

    if (edge_shadow && end_flag == 1u) {
        let Lv = safe_norm(world_p - u_circle_pos);
        world_p += Lv * 2500.0;
    }
    // transform final position into clip space for rendering.

    out.clip_pos = u_view_proj * vec4<f32>(world_p  , 1.0);
    return out;
}

@fragment
fn fragment_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
