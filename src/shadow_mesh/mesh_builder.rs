
use bevy::prelude::*;
use bevy::mesh::{PrimitiveTopology, Indices};
use bevy::asset::RenderAssetUsages;
use crate::shadow_mesh::mesh_attributes::get_pos_from_mesh;
use crate::materials::shadow_material::*;

#[derive(Default, Clone, Debug)]
pub struct MeshBuilder2d {
    pub positions: Option<Vec<[f32; 3]>>,
    pub uvs: Option<Vec<[f32; 2]>>,
    pub normals: Option<Vec<[f32; 3]>>,
    pub indices: Option<Vec<u32>>,
    pub topology: PrimitiveTopology,

    pub edge_n: Option<Vec<[f32; 2]>>,
    pub edge_m: Option<Vec<[f32; 2]>>,
    pub end_flag: Option<Vec<u32>>,
}


impl MeshBuilder2d {
    
    pub fn from_polygon_shadow_quads<'a>(
            mesh: &Mesh
        ) -> Self {
        
        let pts = get_pos_from_mesh(mesh).unwrap();
        assert!(pts.len() >= 3, "polygon needs >= 3 points");

        let edge_count = pts.len();
        let vert_count = edge_count * 4;
        let idx_count  = edge_count * 6;

        let mut pos: Vec<[f32; 3]> = Vec::with_capacity(vert_count);
        let mut uv:  Vec<[f32; 2]> = Vec::with_capacity(vert_count);
        let mut nor: Vec<[f32; 3]> = Vec::with_capacity(vert_count);

        let mut edge_n: Vec<[f32; 2]> = Vec::with_capacity(vert_count);
        let mut edge_m: Vec<[f32; 2]> = Vec::with_capacity(vert_count);
        let mut end_flag: Vec<u32>    = Vec::with_capacity(vert_count);

        let mut indices: Vec<u32> = Vec::with_capacity(idx_count);

            for ei in 0..edge_count {
    let a = ei;
    let b = (ei + 1) % edge_count;

    let a2 = Vec2::new(pts[a][0], pts[a][1]);
    let b2 = Vec2::new(pts[b][0], pts[b][1]);

    let e = b2 - a2;

    let mut n = Vec2::new(-e.y, e.x);
    if n.length_squared() > 1e-12 {
        n = n.normalize();
    } else {
        n = Vec2::ZERO;
    }

    let m = (a2 + b2) * 0.5;

    pos.push([a2.x, a2.y, 0.0]); uv.push([0.0, 0.0]); nor.push([0.0, 0.0, 1.0]);
    edge_n.push([n.x, n.y]); edge_m.push([m.x, m.y]); end_flag.push(0);

    pos.push([b2.x, b2.y, 0.0]); uv.push([1.0, 0.0]); nor.push([0.0, 0.0, 1.0]);
    edge_n.push([n.x, n.y]); edge_m.push([m.x, m.y]); end_flag.push(0);

    pos.push([b2.x, b2.y, 0.0]); uv.push([1.0, 1.0]); nor.push([0.0, 0.0, 1.0]);
    edge_n.push([n.x, n.y]); edge_m.push([m.x, m.y]); end_flag.push(1);

    pos.push([a2.x, a2.y, 0.0]); uv.push([0.0, 1.0]); nor.push([0.0, 0.0, 1.0]);
    edge_n.push([n.x, n.y]); edge_m.push([m.x, m.y]); end_flag.push(1);

    let base = (ei as u32) * 4;
    indices.extend_from_slice(&[
        base + 0, base + 1, base + 2,
        base + 0, base + 2, base + 3,
    ]);
}


        Self {
            topology: PrimitiveTopology::TriangleList,
            positions: Some(pos),
            normals: Some(nor),
            uvs: Some(uv),
            indices: Some(indices),
            edge_n: Some(edge_n),
            edge_m: Some(edge_m),
            end_flag: Some(end_flag),
        }
    }



    //build mesh
    pub fn build(self) -> Mesh {
        let mut mesh = Mesh::new(self.topology, RenderAssetUsages::default());

        if let Some(v) = self.positions { mesh .insert_attribute(Mesh::ATTRIBUTE_POSITION, v); }
        if let Some(v) = self.normals   { mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, v); }
        if let Some(v) = self.uvs       { mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, v); }

        if let Some(v) = self.edge_n    { mesh.insert_attribute(ATTRIBUTE_EDGE_N, v); }
        if let Some(v) = self.edge_m    { mesh.insert_attribute(ATTRIBUTE_EDGE_M, v); }
        if let Some(v) = self.end_flag  { mesh.insert_attribute(ATTRIBUTE_END_FLAG, v); }

        if let Some(idx) = self.indices { mesh.insert_indices(Indices::U32(idx)); }

        mesh
    }
}   
