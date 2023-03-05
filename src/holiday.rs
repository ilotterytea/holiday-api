use serde::{Deserialize, Serialize};

/// Holiday.
#[derive(Deserialize, Serialize, Clone)]
pub struct Holiday {
    pub name: String,
    pub date: (u32, u32),
}
