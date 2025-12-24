use super::Pass;

/// Pipeline = 多个pass
/// Pass ≈ “状态大致一致、可以批处理的阶段”
/// 同一个 Pass 内，GPU 状态“基线一致”，但允许被 Material 覆盖
pub struct Pipeline {
    pub(crate) passes: Vec<Pass>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    pub fn insert(&mut self, pass: Pass) {
        self.passes.push(pass);
        self.sort_passes();
    }

    fn sort_passes(&mut self) {
        self.passes.sort_by(|a, b| {
            a.priority
                .cmp(&b.priority)
                .then_with(|| a.id.raw().cmp(&b.id.raw()))
        });
    }
}
