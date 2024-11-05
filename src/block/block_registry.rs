use askama::Template;

use super::Block;

#[derive(Clone)]
pub struct BlockRegistry<'a> {
    pub blocks: Vec<Block<'a>>,
    pub block_atlas: Vec<BlockAtlasEntry>,
    pub terrain_atlas: Vec<TerrainAtlasEntry>,
}

#[derive(Template)]
#[template(path = "block_serialization/blocks.json.jinja2", escape = "none")]
pub struct BlockAtlasTemplate {
    pub content: String,
}

#[derive(Template)]
#[template(
    path = "block_serialization/terrain_texture.json.jinja2",
    escape = "none"
)]
pub struct TerrainAtlasTemplate {
    pub content: String,
    pub pack_name: String,
}

#[derive(Template)]
#[template(
    path = "block_serialization/block_atlas_entry.json.jinja2",
    escape = "none"
)]
struct BlockAtlasEntryTemplate {
    pub id: String,
    pub textures: String,
    pub sound: String,
}

#[derive(Clone)]
pub struct BlockAtlasEntry {
    pub id: String,
    pub path: String,
    pub textures: String,
    pub sound: String,
}

#[derive(Clone)]
pub struct TerrainAtlasEntry {
    pub id: String,
    pub texture_path: String,
}

impl TerrainAtlasEntry {
    fn serialize(&self) -> String {
        let id: String = (*self)
            .clone()
            .id
            .chars()
            .map(|x| if x == ':' { '_' } else { x })
            .collect();
        TerrainAtlasEntryTemplate {
            texture_path: format!("textures/blocks/{}", id),
            id,
        }
        .render()
        .unwrap()
    }
}

#[derive(Template)]
#[template(
    path = "block_serialization/terrain_texture_entry.json.jinja2",
    escape = "none"
)]
struct TerrainAtlasEntryTemplate {
    pub id: String,
    pub texture_path: String,
}
impl BlockAtlasEntry {
    fn serialize(&self) -> String {
        BlockAtlasEntryTemplate {
            textures: self
                .clone()
                .id
                .chars()
                .map(|x| if x == ':' { '_' } else { x })
                .collect(),
            id: self.clone().id,
            sound: self.clone().sound,
        }
        .render()
        .unwrap()
    }
}

pub fn serialize_block_atlas(atlas: &Vec<BlockAtlasEntry>) -> String {
    let mut atlas_string = String::new();
    for entry in atlas {
        atlas_string.push_str(&entry.serialize());
        atlas_string.push(',');
    }
    atlas_string.pop();
    atlas_string
}

pub fn serialize_terrain_atlas(atlas: &Vec<TerrainAtlasEntry>) -> String {
    let mut atlas_string = String::new();
    for entry in atlas {
        atlas_string.push_str(&entry.serialize());
        atlas_string.push(',');
    }
    atlas_string.pop();
    atlas_string
}

impl<'a> BlockRegistry<'a> {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            block_atlas: vec![],
            terrain_atlas: vec![],
        }
    }

    pub fn add_block(&mut self, block: Block<'a>) {
        self.blocks.push(block.clone());
        self.block_atlas.push(BlockAtlasEntry {
            id: block.clone().type_id.render(),
            path: block.clone().texture_set,
            textures: block
                .type_id
                .render()
                .chars()
                .into_iter()
                .map(|x| if x == ':' { '_' } else { x })
                .collect(),
            sound: block.clone().sound,
        });
        self.terrain_atlas.push(TerrainAtlasEntry {
            id: block.clone().type_id.render(),
            texture_path: block.clone().texture_set,
        });
    }
}
