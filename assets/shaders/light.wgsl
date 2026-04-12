
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var mask_tex: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var mask_samp: sampler; //mask-cam sampler from camera-mask-renderer
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> u_light_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> u_shadow_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> u_full_shadow: u32;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> u_light_uv: vec2<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> u_radius: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(7) var<uniform> u_intensity: f32;




struct VOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vertex(@location(0) position: vec3<f32>) -> VOut {
    var o: VOut;
    o.pos = vec4<f32>(position.xy, 0.0, 1.0);
    o.uv = position.xy * 0.5 + vec2<f32>(0.5, 0.5); 
    return o;
}

@fragment
fn fragment(@builtin(position) frag_pos: vec4<f32>) -> @location(0) vec4<f32> {
    let dims = vec2<f32>(textureDimensions(mask_tex));
    var uv = frag_pos.xy / dims;


    let shadow = textureSample(mask_tex, mask_samp, uv).r;
    let vis = 1.0 - shadow;

    let r = max(u_radius, 1e-4);
    let d = distance(uv, u_light_uv);

 
    let t = clamp(1.0 - d / r, 0.0, 1.0);

    let atten = t * t;       
 

    let intensity = u_intensity * atten * vis;

    let a = clamp(intensity, 0.0, 1.0) * u_light_color.a;
    return vec4<f32>(u_light_color.rgb * a, a);
}