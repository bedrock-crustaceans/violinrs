use std::collections::HashMap;
use derive_setters::Setters;
use serde::Serialize;
use crate::vio::{Identifier, MolangStatement};

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
#[derive(Serialize, Setters)]
#[setters(prefix = "using_")]
pub struct BlockDescriptor {
    tags: Option<MolangStatement>,
    name: Option<Identifier>,
    states: HashMap<String, String>
}

impl BlockDescriptor {
    pub fn new_tags(tags: MolangStatement) -> Self {
        Self { tags: Some(tags), name: None, states: HashMap::new() }
    }
    
    pub fn new_name(name: Identifier) -> Self {
        Self {
            tags: None,
            name: Some(name),
            states: HashMap::new()
        }
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BlockRenderMethod {
    AlphaTest,
    AlphaTestSingleSided,
    Blend,
    DoubleSided,
    Opaque
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BlockFace {
    North,
    East,
    South,
    West,
    Up,
    Down
}

#[derive(Clone, Debug, Serialize, Setters)]
#[setters(prefix = "using_")]
pub struct MaterialInstance {
    pub texture: String,
    pub render_method: BlockRenderMethod,
    pub face_dimming: bool,
    pub ambient_occlusion: bool,
}

impl MaterialInstance {
    pub fn new(texture: impl Into<String>, render_method: BlockRenderMethod) -> Self {
        Self {
            texture: texture.into(),
            render_method,
            face_dimming: true,
            ambient_occlusion: true
        }
    }
}

#[derive(Clone, Debug, Serialize, Setters)]
#[setters(prefix = "using_")]
pub struct BlockPlacementCondition {
    allowed_faces: Vec<BlockFace>,
    block_filter: Vec<BlockDescriptor>
}

impl BlockPlacementCondition {
    pub fn new() -> Self {
        Self {
            allowed_faces: Vec::new(),
            block_filter: Vec::new()
        }
    }
}

