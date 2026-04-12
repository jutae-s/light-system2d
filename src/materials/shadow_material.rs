use bevy::{
    prelude::*,
    asset::Asset,
    reflect::TypePath,
    color::LinearRgba,
    shader::ShaderRef,
    mesh::{Mesh, MeshVertexAttribute, MeshVertexBufferLayoutRef, VertexFormat},
    render::render_resource::{AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError,},
    sprite_render::{Material2d, Material2dKey},
};



pub const SHADER_PATH: &str = "shaders/custom-shader.wgsl";

pub const ATTRIBUTE_EDGE_N: MeshVertexAttribute =
    MeshVertexAttribute::new("EdgeN", 110, VertexFormat::Float32x2);
pub const ATTRIBUTE_EDGE_M: MeshVertexAttribute =
    MeshVertexAttribute::new("EdgeM", 111, VertexFormat::Float32x2);
pub const ATTRIBUTE_END_FLAG: MeshVertexAttribute =
    MeshVertexAttribute::new("EndFlag", 112, VertexFormat::Uint32);


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    pub color: LinearRgba,

    #[uniform(1)]
    pub time: f32,

    #[uniform(2)]
    pub transform: Mat4,

    #[uniform(3)]
    pub view_proj: Mat4,

    #[uniform(4)]
    pub obj_pos: Vec3
}


impl Material2d for CustomMaterial{

    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }


   fn vertex_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
 
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> 
    {
        let vertex_layout = layout.0.get_layout(&[
        Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
        Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
        Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
        ATTRIBUTE_EDGE_N.at_shader_location(3),
        ATTRIBUTE_EDGE_M.at_shader_location(4),
        ATTRIBUTE_END_FLAG.at_shader_location(5),
    ])?;
    descriptor.vertex.buffers = vec![vertex_layout];
    Ok(())
    }
 
}