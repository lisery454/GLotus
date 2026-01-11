mod instance_buffer;
mod mesh;
mod mesh_error;

use glam::{Vec2, Vec3};
pub use instance_buffer::*;
pub use mesh::*;
pub use mesh_error::*;

use slotmap::{SlotMap, new_key_type};

new_key_type! {
    pub struct MeshHandle;
}

pub struct MeshManager {
    meshes: SlotMap<MeshHandle, Mesh>,
}

impl MeshManager {
    pub fn new() -> Self {
        Self {
            meshes: SlotMap::with_key(),
        }
    }
    pub(crate) fn get(&self, handle: MeshHandle) -> Option<&Mesh> {
        self.meshes.get(handle)
    }

    pub(crate) fn get_mut(&mut self, handle: MeshHandle) -> Option<&mut Mesh> {
        self.meshes.get_mut(handle)
    }

    /// 从位置数据创建mesh
    pub fn create_from_positions(
        &mut self,
        indices: Vec<u32>,
        positions: Vec<f32>,
    ) -> Result<MeshHandle, MeshError> {
        if positions.len() % 3 != 0 {
            return Err(MeshError::InvalidData(
                "Positions length must be divisible by 3".to_string(),
            ));
        }

        let positions: Vec<Vec3> = positions
            .chunks_exact(3)
            .map(|p| Vec3::new(p[0], p[1], p[2]))
            .collect();

        let vertex_data = VertexData::new(positions);
        let mesh = Mesh::from_data(vertex_data, indices).map_err(|e| MeshError::InvalidData(e))?;

        Ok(self.meshes.insert(mesh))
    }

    /// 从位置和法线数据创建mesh
    pub fn create_from_positions_normals(
        &mut self,
        indices: Vec<u32>,
        positions: Vec<f32>,
        normals: Vec<f32>,
    ) -> Result<MeshHandle, MeshError> {
        if positions.len() % 3 != 0 || normals.len() % 3 != 0 {
            return Err(MeshError::InvalidData(
                "Positions and normals length must be divisible by 3".to_string(),
            ));
        }

        if positions.len() != normals.len() {
            return Err(MeshError::InvalidData(
                "Positions and normals must have the same length".to_string(),
            ));
        }

        let positions: Vec<Vec3> = positions
            .chunks_exact(3)
            .map(|p| Vec3::new(p[0], p[1], p[2]))
            .collect();

        let normals: Vec<Vec3> = normals
            .chunks_exact(3)
            .map(|n| Vec3::new(n[0], n[1], n[2]))
            .collect();

        let vertex_data = VertexData::new(positions).with_normals(normals);
        let mesh = Mesh::from_data(vertex_data, indices).map_err(|e| MeshError::InvalidData(e))?;

        Ok(self.meshes.insert(mesh))
    }

    /// 从位置和UV坐标创建mesh
    pub fn create_from_positions_uvs(
        &mut self,
        indices: Vec<u32>,
        positions: Vec<f32>,
        uvs: Vec<f32>,
    ) -> Result<MeshHandle, MeshError> {
        if positions.len() % 3 != 0 {
            return Err(MeshError::InvalidData(
                "Positions length must be divisible by 3".to_string(),
            ));
        }

        if uvs.len() % 2 != 0 {
            return Err(MeshError::InvalidData(
                "UVs length must be divisible by 2".to_string(),
            ));
        }

        let vertex_count = positions.len() / 3;
        if uvs.len() / 2 != vertex_count {
            return Err(MeshError::InvalidData(
                "UVs count must match vertex count".to_string(),
            ));
        }

        let positions: Vec<Vec3> = positions
            .chunks_exact(3)
            .map(|p| Vec3::new(p[0], p[1], p[2]))
            .collect();

        let uvs: Vec<Vec2> = uvs
            .chunks_exact(2)
            .map(|uv| Vec2::new(uv[0], uv[1]))
            .collect();

        let vertex_data = VertexData::new(positions).with_uvs(uvs);
        let mesh = Mesh::from_data(vertex_data, indices).map_err(|e| MeshError::InvalidData(e))?;

        Ok(self.meshes.insert(mesh))
    }

    /// 从位置、法线和UV坐标创建mesh
    pub fn create_from_positions_normals_uvs(
        &mut self,
        indices: Vec<u32>,
        positions: Vec<f32>,
        normals: Vec<f32>,
        uvs: Vec<f32>,
    ) -> Result<MeshHandle, MeshError> {
        if positions.len() % 3 != 0 || normals.len() % 3 != 0 {
            return Err(MeshError::InvalidData(
                "Positions and normals length must be divisible by 3".to_string(),
            ));
        }

        if uvs.len() % 2 != 0 {
            return Err(MeshError::InvalidData(
                "UVs length must be divisible by 2".to_string(),
            ));
        }

        let vertex_count = positions.len() / 3;
        if normals.len() / 3 != vertex_count || uvs.len() / 2 != vertex_count {
            return Err(MeshError::InvalidData(
                "All attributes must have matching vertex counts".to_string(),
            ));
        }

        let positions: Vec<Vec3> = positions
            .chunks_exact(3)
            .map(|p| Vec3::new(p[0], p[1], p[2]))
            .collect();

        let normals: Vec<Vec3> = normals
            .chunks_exact(3)
            .map(|n| Vec3::new(n[0], n[1], n[2]))
            .collect();

        let uvs: Vec<Vec2> = uvs
            .chunks_exact(2)
            .map(|uv| Vec2::new(uv[0], uv[1]))
            .collect();

        let vertex_data = VertexData::new(positions)
            .with_normals(normals)
            .with_uvs(uvs);

        let mesh = Mesh::from_data(vertex_data, indices).map_err(|e| MeshError::InvalidData(e))?;

        Ok(self.meshes.insert(mesh))
    }

    /// 从完整顶点数据创建mesh (支持所有属性)
    pub fn create_from_vertex_data(
        &mut self,
        indices: Vec<u32>,
        vertex_data: VertexData,
    ) -> Result<MeshHandle, MeshError> {
        let mesh = Mesh::from_data(vertex_data, indices).map_err(|e| MeshError::InvalidData(e))?;

        Ok(self.meshes.insert(mesh))
    }

    /// 从OBJ文件路径创建mesh
    pub fn create_from_obj_path(&mut self, path: &str) -> Result<MeshHandle, MeshError> {
        let mesh = Mesh::from_obj(path).map_err(|e| MeshError::LoadError(e))?;

        Ok(self.meshes.insert(mesh))
    }

    /// 从内存中的OBJ数据创建mesh
    pub fn create_from_obj_bytes(&mut self, data: &[u8]) -> Result<MeshHandle, MeshError> {
        // 将字节转换为字符串
        let obj_content = std::str::from_utf8(data)
            .map_err(|e| MeshError::LoadError(format!("Invalid UTF-8: {}", e)))?;

        // 使用tobj从字符串加载
        let (models, _) = tobj::load_obj_buf(
            &mut obj_content.as_bytes(),
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
            |_| Ok((vec![], Default::default())),
        )
        .map_err(|e| MeshError::LoadError(format!("Failed to parse OBJ: {}", e)))?;

        if models.is_empty() {
            return Err(MeshError::LoadError(
                "No models found in OBJ data".to_string(),
            ));
        }

        // 使用第一个模型
        let mesh_data = &models[0].mesh;

        let positions: Vec<Vec3> = mesh_data
            .positions
            .chunks_exact(3)
            .map(|p| Vec3::new(p[0], p[1], p[2]))
            .collect();

        let mut vertex_data = VertexData::new(positions);

        if !mesh_data.normals.is_empty() {
            let normals: Vec<Vec3> = mesh_data
                .normals
                .chunks_exact(3)
                .map(|n| Vec3::new(n[0], n[1], n[2]))
                .collect();
            vertex_data = vertex_data.with_normals(normals);
        }

        if !mesh_data.texcoords.is_empty() {
            let uvs: Vec<Vec2> = mesh_data
                .texcoords
                .chunks_exact(2)
                .map(|uv| Vec2::new(uv[0], uv[1]))
                .collect();
            vertex_data = vertex_data.with_uvs(uvs);
        }

        let mesh = Mesh::from_data(vertex_data, mesh_data.indices.clone())
            .map_err(|e| MeshError::InvalidData(e))?;

        Ok(self.meshes.insert(mesh))
    }

    /// 移除mesh
    pub fn remove(&mut self, handle: MeshHandle) -> Option<Mesh> {
        self.meshes.remove(handle)
    }

    /// 检查handle是否有效
    pub fn contains(&self, handle: MeshHandle) -> bool {
        self.meshes.contains_key(handle)
    }

    /// 获取管理的mesh数量
    pub fn len(&self) -> usize {
        self.meshes.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.meshes.is_empty()
    }

    /// 清空所有mesh
    pub fn clear(&mut self) {
        self.meshes.clear();
    }

    /// 迭代所有mesh
    pub fn iter(&self) -> impl Iterator<Item = (MeshHandle, &Mesh)> {
        self.meshes.iter()
    }

    /// 可变迭代所有mesh
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (MeshHandle, &mut Mesh)> {
        self.meshes.iter_mut()
    }
}
