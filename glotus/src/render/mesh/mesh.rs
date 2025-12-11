use std::{cell::RefCell, rc::Rc};

use cgmath::{Vector2, Vector3};

use crate::render::mesh::MeshError;

pub struct Mesh {
    pub positions: Vec<Vector3<f32>>, // 某个位置索引对应的位置
    pub normals: Vec<Vector3<f32>>,   // 某个法线索引对应的法线
    pub texcoords: Vec<Vector2<f32>>, // 某个uv索引对应的uv
    pub position_indexs: Vec<usize>,  // 某个顶点的位置索引
    pub normal_indexs: Vec<usize>,    // 某个顶点的法线索引
    pub texcoord_indexs: Vec<usize>,  // 某个顶点的uv索引
    pub count: usize,                 // 一共多少个顶点
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            positions: Default::default(),
            normals: Default::default(),
            texcoords: Default::default(),
            position_indexs: Default::default(),
            normal_indexs: Default::default(),
            texcoord_indexs: Default::default(),
            count: Default::default(),
        }
    }
}

impl Mesh {
    pub fn from_position(positions: &Vec<f32>, position_indexs: &Vec<u32>) -> Rc<RefCell<Self>> {
        let count = position_indexs.len();
        Rc::new(RefCell::new(Self {
            positions: positions
                .chunks(3)
                .map(|c| Vector3::new(c[0], c[1], c[2]))
                .collect(),
            position_indexs: position_indexs.iter().map(|c| *c as usize).collect(),
            count,
            ..Default::default()
        }))
    }

    pub fn from_position_normal(
        positions: &Vec<f32>,
        position_indexs: &Vec<u32>,
        normals: &Vec<f32>,
        normal_indexs: &Vec<u32>,
    ) -> Rc<RefCell<Self>> {
        let count = position_indexs.len();
        assert_eq!(count, normal_indexs.len());

        Rc::new(RefCell::new(Self {
            positions: positions
                .chunks(3)
                .map(|c| Vector3::new(c[0], c[1], c[2]))
                .collect(),
            position_indexs: position_indexs.iter().map(|c| *c as usize).collect(),
            normals: normals
                .chunks(3)
                .map(|c| Vector3::new(c[0], c[1], c[2]))
                .collect(),
            normal_indexs: normal_indexs.iter().map(|c| *c as usize).collect(),
            count,
            ..Default::default()
        }))
    }

    pub fn from_position_texcoord(
        positions: &Vec<f32>,
        position_indexs: &Vec<u32>,
        texcoords: &Vec<f32>,
        texcoord_indexs: &Vec<u32>,
    ) -> Rc<RefCell<Self>> {
        let count = position_indexs.len();
        assert_eq!(count, texcoord_indexs.len());

        Rc::new(RefCell::new(Self {
            positions: positions
                .chunks(3)
                .map(|c| Vector3::new(c[0], c[1], c[2]))
                .collect(),
            position_indexs: position_indexs.iter().map(|c| *c as usize).collect(),
            texcoords: texcoords
                .chunks(2)
                .map(|c| Vector2::new(c[0], c[1]))
                .collect(),
            texcoord_indexs: texcoord_indexs.iter().map(|c| *c as usize).collect(),
            count,
            ..Default::default()
        }))
    }

    pub fn from_position_normal_texcoord(
        positions: &Vec<f32>,
        position_indexs: &Vec<u32>,
        normals: &Vec<f32>,
        normal_indexs: &Vec<u32>,
        texcoords: &Vec<f32>,
        texcoord_indexs: &Vec<u32>,
    ) -> Rc<RefCell<Self>> {
        let count = position_indexs.len();
        assert_eq!(count, normal_indexs.len());
        assert_eq!(count, texcoord_indexs.len());

        Rc::new(RefCell::new(Self {
            positions: positions
                .chunks(3)
                .map(|c| Vector3::new(c[0], c[1], c[2]))
                .collect(),
            position_indexs: position_indexs.iter().map(|c| *c as usize).collect(),
            normals: normals
                .chunks(3)
                .map(|c| Vector3::new(c[0], c[1], c[2]))
                .collect(),
            normal_indexs: normal_indexs.iter().map(|c| *c as usize).collect(),
            texcoords: texcoords
                .chunks(2)
                .map(|c| Vector2::new(c[0], c[1]))
                .collect(),
            texcoord_indexs: texcoord_indexs.iter().map(|c| *c as usize).collect(),
            count,
            ..Default::default()
        }))
    }

    pub fn load_obj(path: &str) -> Result<Rc<RefCell<Self>>, MeshError> {
        let (models, _) = tobj::load_obj(
            path,
            &tobj::LoadOptions {
                triangulate: true,
                ..Default::default()
            },
        )
        .map_err(|_| MeshError::TObjLoadFail)?;

        let mesh = &models[0].mesh;

        Ok(Self::from_position_normal_texcoord(
            &mesh.positions,
            &mesh.indices,
            &mesh.normals,
            &mesh.normal_indices,
            &mesh.texcoords,
            &mesh.texcoord_indices,
        ))
    }
}
