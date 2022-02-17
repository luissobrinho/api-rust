pub mod item;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id {
    pub(crate) name: String,
}