use super::{BlendMode, CullFaceMode, DepthMode, PolygonMode, StencilMode};

#[derive(Clone, Debug, Default)]
pub struct RenderState {
    depth_mode: DepthMode,
    stencil_mode: StencilMode,
    blend_mode: BlendMode,
    cull_face_mode: CullFaceMode,
    polygon_mode: PolygonMode,
}

impl RenderState {
    pub fn new(
        depth_mode: DepthMode,
        stencil_mode: StencilMode,
        blend_mode: BlendMode,
        cull_face_mode: CullFaceMode,
        polygon_mode: PolygonMode,
    ) -> Self {
        Self {
            depth_mode,
            stencil_mode,
            blend_mode,
            cull_face_mode,
            polygon_mode,
        }
    }

    pub fn apply(&self) {
        self.depth_mode.apply();
        self.stencil_mode.apply();
        self.blend_mode.apply();
        self.cull_face_mode.apply();
        self.polygon_mode.apply();
    }

    pub fn get_polygon_mode(&self) -> PolygonMode {
        self.polygon_mode
    }
    pub fn set_polygon_mode(&mut self, mode: PolygonMode) {
        if mode != self.polygon_mode {
            self.polygon_mode = mode;
            self.polygon_mode.apply();
        }
    }

    pub fn get_cull_face_mode(&self) -> CullFaceMode {
        self.cull_face_mode
    }
    pub fn set_cull_face_mode(&mut self, mode: CullFaceMode) {
        if mode != self.cull_face_mode {
            self.cull_face_mode = mode;
            self.cull_face_mode.apply();
        }
    }

    pub fn get_blend_mode(&self) -> BlendMode {
        self.blend_mode
    }
    pub fn set_blend_mode(&mut self, mode: BlendMode) {
        if mode != self.blend_mode {
            self.blend_mode = mode;
            self.blend_mode.apply();
        }
    }

    pub fn get_stencil_mode(&self) -> StencilMode {
        self.stencil_mode
    }
    pub fn set_stencil_mode(&mut self, mode: StencilMode) {
        if mode != self.stencil_mode {
            self.stencil_mode = mode;
            self.stencil_mode.apply();
        }
    }

    pub fn get_depth_mode(&self) -> DepthMode {
        self.depth_mode
    }
    pub fn set_depth_mode(&mut self, mode: DepthMode) {
        if mode != self.depth_mode {
            self.depth_mode = mode;
            self.depth_mode.apply();
        }
    }
}
