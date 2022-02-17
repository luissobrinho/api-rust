use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

type Items = HashMap<String, i32>;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    pub(crate) name: String,
    pub(crate) finish: bool,
}

#[derive(Clone)]
pub struct Store {
    pub(crate) todo_list: Arc<RwLock<Items>>
}

impl Store {
    pub(crate) fn new() -> Self {
        Store {
            todo_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}