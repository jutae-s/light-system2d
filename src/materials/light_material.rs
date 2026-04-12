use bevy::{
    prelude::*,
    asset::Asset,
    reflect::TypePath,
    color::LinearRgba,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d},
};



pub const LIGHT_SHADER_PATH: &str = "shaders/light.wgsl";


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LightMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub mask: Handle<Image>,

    #[uniform(2)]
    pub light_color: LinearRgba,

    #[uniform(3)]
    pub shadow_color: LinearRgba,

    #[uniform(4)]
    pub full_shadow: u32,


    #[uniform(5)]
    pub shadow_uv: Vec2,

    #[uniform(6)]
    pub light_radius: f32,


   #[uniform(7)]
    pub intenisty: f32,


    


}


impl Material2d for LightMaterial {
    fn vertex_shader() -> ShaderRef { LIGHT_SHADER_PATH.into() }
    fn fragment_shader() -> ShaderRef { LIGHT_SHADER_PATH.into() }
        fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
