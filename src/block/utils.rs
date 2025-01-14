use serde::Serialize;
use crate::vio::{MolangStatement};

#[derive(Clone, Debug)]
#[derive(Serialize)]
pub struct BlockDestroySpeed {
    block: BlockDescriptor,
    speed: i32
}

impl BlockDestroySpeed {
    pub fn new(block: BlockDescriptor, speed: i32) -> Self {
        Self { block, speed }
    }
}

#[derive(Clone, Debug)]
#[derive(Serialize)]
pub struct BlockDescriptor {
    tags: MolangStatement
}

impl BlockDescriptor {
    pub fn new(tags: MolangStatement) -> Self {
        Self { tags }
    }
}

