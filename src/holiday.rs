use serde::{Deserialize, Serialize};

/// Holiday.
#[derive(Deserialize, Serialize)]
pub struct Holiday {
    pub name: String,
    pub date: (usize, usize),
}
