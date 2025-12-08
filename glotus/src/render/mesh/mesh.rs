use std::{cell::RefCell, rc::Rc};

use cgmath::{Vector2, Vector3};

use super::vertex::Vertex;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { vertices, indices }))
    }

    pub fn load_obj(path: &str) -> Rc<RefCell<Self>> {
        let (models, _) = tobj::load_obj(
            path,
            &tobj::LoadOptions {
                triangulate: true,
                ..Default::default()
            },
        )
        .unwrap();

        let mesh = &models[0].mesh;

        let mut vertices = Vec::new();

        for i in 0..mesh.indices.len() {
            let position_index = mesh.indices[i] as usize;

            let position = Vector3::new(
                mesh.positions[position_index * 3],
                mesh.positions[position_index * 3 + 1],
                mesh.positions[position_index * 3 + 2],
            );

            // 法线要检查是否存在
            let normal = if !mesh.normals.is_empty() {
                let normal_index = mesh.normal_indices[i] as usize;
                Vector3::new(
                    mesh.normals[normal_index * 3],
                    mesh.normals[normal_index * 3 + 1],
                    mesh.normals[normal_index * 3 + 2],
                )
            } else {
                Vector3::new(0.0, 1.0, 0.0)
            };

            // UV 要检查 size
            let tex_coord = if !mesh.texcoords.is_empty() {
                let texture_index = mesh.texcoord_indices[i] as usize;
                Vector2::new(
                    mesh.texcoords[texture_index * 2],
                    mesh.texcoords[texture_index * 2 + 1],
                )
            } else {
                Vector2::new(0.0, 0.0)
            };

            vertices.push(Vertex {
                position,
                normal,
                tex_coord,
                tangent: Vector3::new(0.0, 0.0, 0.0),
                bitangent: Vector3::new(0.0, 0.0, 0.0),
                color: Vector3::new(1.0, 1.0, 1.0),
            });
        }

        Self::new(
            vertices,
            (0..mesh.indices.len())
                .map(|x| x as u32)
                .collect::<Vec<u32>>(),
        )
    }

    pub fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }
}
