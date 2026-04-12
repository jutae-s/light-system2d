use bevy::prelude::*;
use bevy::mesh::{VertexAttributeValues, Indices};
use crate::shadow_mesh::mesh_builder::MeshBuilder2d;

pub fn get_attributes<'a>(
    meshes: &'a Assets<Mesh>,
    mesh_handle: &Handle<Mesh>,
) -> Option<MeshBuilder2d> {
    let mesh_handle = meshes.get(mesh_handle)?;

    let mut out = MeshBuilder2d {
        topology: mesh_handle.primitive_topology(),
        ..Default::default()
    };
     if let Some(vals) = mesh_handle.attribute(Mesh::ATTRIBUTE_POSITION) {
        out.positions = match vals {
            VertexAttributeValues::Float32x2(v) => {
                Some(v.iter().map(|p| [p[0], p[1], 0.0]).collect())
            }
            _ => None,
        };
    }

    if let Some(vals) = mesh_handle.attribute(Mesh::ATTRIBUTE_NORMAL) {
        out.normals = match vals {
            VertexAttributeValues::Float32x3(v) => Some(v.clone()),
            _ => None,
        };
    }

    if let Some(vals) = mesh_handle.attribute(Mesh::ATTRIBUTE_UV_0) {
        out.uvs = match vals {
            VertexAttributeValues::Float32x2(v) => Some(v.clone()),
            _ => None,
        };
    }

    out.indices = mesh_handle.indices().map(|idx| match idx {
        Indices::U16(v) => v.iter().map(|&i| i as u32).collect(),
        Indices::U32(v) => v.clone(),
    });

    Some(out)
}


pub fn get_pos_from_mesh(mesh: &Mesh) -> Option<Vec<[f32; 3]>> {
    let vals = mesh.attribute(Mesh::ATTRIBUTE_POSITION)?;
    match vals {
        VertexAttributeValues::Float32x3(v) => Some(v.clone()),
        VertexAttributeValues::Float32x2(v) => Some(v.iter().map(|p| [p[0], p[1], 0.0]).collect()),
        _ => None,
    }
}




