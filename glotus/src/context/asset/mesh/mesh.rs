use std::io::Cursor;

use cgmath::{Vector2, Vector3};

use super::MeshError;

/// 网格模型
pub struct Mesh {
    pub(crate) positions: Vec<Vector3<f32>>, // 某个位置索引对应的位置
    pub(crate) normals: Vec<Vector3<f32>>,   // 某个法线索引对应的法线
    pub(crate) texcoords: Vec<Vector2<f32>>, // 某个uv索引对应的uv
    pub(crate) indices: Vec<usize>,          // 所有顶点
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            positions: Default::default(),
            normals: Default::default(),
            texcoords: Default::default(),
            indices: Default::default(),
        }
    }
}

impl Mesh {
    /// 从位置数组生成
    pub fn from_position(indices: &Vec<usize>, positions: &Vec<f32>) -> Self {
        let count = positions.len();
        assert_eq!(count % 3, 0);
        Self {
            indices: indices.clone(),
            positions: positions
                .chunks(3)
                .map(|c| Vector3::new(c[0], c[1], c[2]))
                .collect(),
            ..Default::default()
        }
    }

    /// 从位置数组和法线数组生成
    pub fn from_position_normal(
        indices: &Vec<usize>,
        positions: &Vec<f32>,
        normals: &Vec<f32>,
    ) -> Self {
        let positions_count = positions.len();
        assert_eq!(positions_count % 3, 0);
        let normals_count = normals.len();
        assert_eq!(normals_count % 3, 0);
        assert_eq!(normals_count / 3, positions_count / 3);

        Self {
            indices: indices.clone(),
            positions: positions
                .chunks(3)
                .map(|c| Vector3::new(c[0], c[1], c[2]))
                .collect(),
            normals: normals
                .chunks(3)
                .map(|c| Vector3::new(c[0], c[1], c[2]))
                .collect(),
            ..Default::default()
        }
    }

    /// 从位置数组和UV数组生成
    pub fn from_position_texcoord(
        indices: &Vec<usize>,
        positions: &Vec<f32>,
        texcoords: &Vec<f32>,
    ) -> Self {
        let positions_count = positions.len();
        assert_eq!(positions_count % 3, 0);
        let texcoords_count = texcoords.len();
        assert_eq!(texcoords_count % 2, 0);
        assert_eq!(texcoords_count / 2, positions_count / 3);

        Self {
            indices: indices.clone(),
            positions: positions
                .chunks(3)
                .map(|c| Vector3::new(c[0], c[1], c[2]))
                .collect(),
            texcoords: texcoords
                .chunks(2)
                .map(|c| Vector2::new(c[0], c[1]))
                .collect(),
            ..Default::default()
        }
    }

    /// 从位置，法线，UV数组生成
    pub fn from_position_normal_texcoord(
        indices: &Vec<usize>,
        positions: &Vec<f32>,
        normals: &Vec<f32>,
        texcoords: &Vec<f32>,
    ) -> Self {
        let positions_count = positions.len();
        assert_eq!(positions_count % 3, 0);
        let normals_count = normals.len();
        assert_eq!(normals_count % 3, 0);
        let texcoords_count = texcoords.len();
        assert_eq!(texcoords_count % 2, 0);
        assert_eq!(texcoords_count / 2, positions_count / 3);
        assert_eq!(texcoords_count / 2, normals_count / 3);

        Self {
            indices: indices.clone(),
            positions: positions
                .chunks(3)
                .map(|c| Vector3::new(c[0], c[1], c[2]))
                .collect(),
            normals: normals
                .chunks(3)
                .map(|c| Vector3::new(c[0], c[1], c[2]))
                .collect(),
            texcoords: texcoords
                .chunks(2)
                .map(|c| Vector2::new(c[0], c[1]))
                .collect(),
            ..Default::default()
        }
    }

    /// 从obj文件导入
    pub fn load_obj(path: &str) -> Result<Self, MeshError> {
        let (models, _) = tobj::load_obj(
            path,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
        )
        .map_err(|_| MeshError::TObjLoadFail)?;

        let mesh = &models[0].mesh;

        Ok(Self::from_position_normal_texcoord(
            &mesh.indices.iter().map(|&x| x as usize).collect(),
            &mesh.positions,
            &mesh.normals,
            &mesh.texcoords,
        ))
    }

    /// 从内存的byte加载
    pub fn load_obj_from_memory(data: &[u8]) -> Result<Self, MeshError> {
        // 使用 load_obj_buf 处理字节流
        let (models, _) = tobj::load_obj_buf(
            &mut Cursor::new(data),
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
            |_mtl_path| Err(tobj::LoadError::MaterialParseError), // 默认不处理材质
        )
        .map_err(|_| MeshError::TObjLoadFail)?;

        if models.is_empty() {
            return Err(MeshError::TObjLoadFail);
        }

        let mesh = &models[0].mesh;
        Ok(Self::from_position_normal_texcoord(
            &mesh.indices.iter().map(|&x| x as usize).collect(),
            &mesh.positions,
            &mesh.normals,
            &mesh.texcoords,
        ))
    }
}
