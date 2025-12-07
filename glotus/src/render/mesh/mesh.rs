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
        for i in 0..mesh.positions.len() / 3 {
            vertices.push(Vertex {
                position: Vector3::new(
                    mesh.positions[i * 3],
                    mesh.positions[i * 3 + 1],
                    mesh.positions[i * 3 + 2],
                ),
                normal: if !mesh.normals.is_empty() {
                    Vector3::new(
                        mesh.normals[i * 3],
                        mesh.normals[i * 3 + 1],
                        mesh.normals[i * 3 + 2],
                    )
                } else {
                    Vector3::new(0.0, 0.0, 0.0)
                },
                tex_coord: if !mesh.texcoords.is_empty() {
                    Vector2::new(mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1])
                } else {
                    Vector2::new(0.0, 0.0)
                },
                tangent: Vector3::new(0.0, 0.0, 0.0),
                bitangent: Vector3::new(0.0, 0.0, 0.0),
                color: Vector3::new(1.0, 1.0, 1.0),
            });
        }

        Self::new(vertices, mesh.indices.clone())
    }

    pub fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }
}
