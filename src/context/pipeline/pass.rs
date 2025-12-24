use super::RenderState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PassId(u32);

const USER_BIT: u32 = 1 << 31;

impl PassId {
    /// 用户 / 命名 pass（稳定，可序列化）
    pub fn named(name: &str) -> Self {
        let hash = fnv1a_32(name.as_bytes());
        PassId(USER_BIT | (hash & !USER_BIT))
    }

    pub fn is_user(self) -> bool {
        (self.0 & USER_BIT) != 0
    }

    pub fn raw(self) -> u32 {
        self.0
    }
}

impl From<&str> for PassId {
    fn from(name: &str) -> Self {
        PassId::named(name)
    }
}

fn fnv1a_32(bytes: &[u8]) -> u32 {
    const FNV_OFFSET: u32 = 0x811C9DC5;
    const FNV_PRIME: u32 = 16777619;

    let mut hash = FNV_OFFSET;
    for b in bytes {
        hash ^= *b as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

pub struct Pass {
    pub id: PassId,
    pub priority: i32,
    pub default_state: RenderState,
}

impl Pass {
    pub fn new(id: PassId, priority: i32, state: RenderState) -> Self {
        Self {
            id,
            priority,
            default_state: state,
        }
    }
}
