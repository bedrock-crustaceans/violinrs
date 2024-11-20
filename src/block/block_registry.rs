use std::sync::Arc;
use askama::Template;
use crate::image::Image;
use crate::vio::{Buildable, Identifier};
use super::Block;

#[derive(Clone)]
pub struct BlockRegistry {
    pub blocks: Vec<Block>,
    pub block_atlas: Vec<Arc<dyn BlockAtlasEntry>>,
    pub terrain_atlas: Vec<TerrainAtlasEntry>,
    pub textures: Vec<BlockTexture>
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
struct AllBlockAtlasEntryTemplate {
    pub id: String,
    pub textures: String,
    pub sound: String,
}

pub trait BlockAtlasEntry {
    fn serialize(&self) -> String;
    fn id(&self) -> Identifier;
}

#[derive(Clone)]
pub struct AllBlockAtlasEntry {
    pub id: Identifier,
    pub textures: String,
    pub sound: String,
}

#[derive(Clone)]
pub struct PerFaceBlockAtlasEntry {
    pub id: Identifier,
    pub textures: Faces,
    pub sound: String,
}

#[derive(Clone)]
pub struct Faces {
    up: String,
    down: String,
    north: String,
    south: String,
    east: String,
    west: String,
}

impl PerFaceBlockAtlasEntry {
    pub fn new(id: Identifier, textures: Faces, sound: impl Into<String>) -> Self {
        Self {
            id, textures, sound: sound.into()
        }
    }
}

impl Faces {
    pub fn new(
        up: impl Into<String>,
        down: impl Into<String>,
        north: impl Into<String>,
        south: impl Into<String>,
        east: impl Into<String>,
        west: impl Into<String>
    ) -> Self {
        Self {
            up: up.into(),
            down: down.into(),
            north: north.into(),
            south: south.into(),
            east: east.into(),
            west: west.into()
        }
    }

    pub fn new_identifiers(
        up: Identifier,
        down: Identifier,
        north: Identifier,
        south: Identifier,
        east: Identifier,
        west: Identifier
    ) -> Self {
        Self {
            up: up.render(),
            down: down.render(),
            north: north.render(),
            south: south.render(),
            east: east.render(),
            west: west.render()
        }
    }
}

impl BlockAtlasEntry for PerFaceBlockAtlasEntry {
    fn serialize(&self) -> String {
        PerFaceBlockAtlasEntryTemplate {
            id: self.id.clone().render(),
            sound: self.sound.clone(),
            up: self.textures.up.clone(),
            down: self.textures.down.clone(),
            north: self.textures.north.clone(),
            south: self.textures.south.clone(),
            east: self.textures.east.clone(),
            west: self.textures.west.clone()
        }.render().unwrap()
    }

    fn id(&self) -> Identifier {
        self.id.clone()
    }
}

#[derive(Template)]
#[template(
    path = "block_serialization/perface_block_entry.json.jinja2",
    escape = "none"
)]
struct PerFaceBlockAtlasEntryTemplate {
    pub up: String,
    pub down: String,
    pub north: String,
    pub south: String,
    pub east: String,
    pub west: String,
    pub id: String,
    pub sound: String,
}

impl AllBlockAtlasEntry {
    pub fn new(id: Identifier, textures: impl Into<String>, sound: impl Into<String>) -> Self {
        Self {
            id, textures: textures.into(), sound: sound.into()
        }
    }
}



#[derive(Clone)]
pub struct TerrainAtlasEntry {
    pub id: String,
    pub texture_path: String,
}

impl TerrainAtlasEntry {
    fn serialize(&self) -> String {
        TerrainAtlasEntryTemplate {
            texture_path: self.texture_path.clone(),
            id: self.id.clone(),
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
impl BlockAtlasEntry for AllBlockAtlasEntry {
    fn serialize(&self) -> String {
        AllBlockAtlasEntryTemplate {
            textures: self.textures.clone(),
            id: self.clone().id.render(),
            sound: self.clone().sound,
        }
        .render()
        .unwrap()
    }

    fn id(&self) -> Identifier {
        self.id.clone()
    }
}

pub fn serialize_block_atlas(atlas: &Vec<Arc<dyn BlockAtlasEntry>>) -> String {
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

impl BlockRegistry {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            block_atlas: vec![],
            terrain_atlas: vec![],
            textures: vec![]
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block.clone());
        // self.block_atlas.push(BlockAtlasEntry {
        //     id: block.clone().type_id.render(),
        //     path: block.clone().texture_set,
        //     textures: block
        //         .type_id
        //         .render()
        //         .chars()
        //         .into_iter()
        //         .map(|x| if x == ':' { '_' } else { x })
        //         .collect(),
        //     sound: block.clone().sound,
        // });
        // self.terrain_atlas.push(TerrainAtlasEntry {
        //     id: block.clone().type_id.render(),
        //     texture_path: block.clone().texture_set,
        // });
    }

    pub fn add_texture(&mut self, texture: BlockTexture) {
        self.textures.push(texture.clone());
    }

    pub fn add_terrain_atlas_entry(&mut self, entry: TerrainAtlasEntry) {
        self.terrain_atlas.push(entry.clone());
    }

    pub fn add_block_atlas_entry(&mut self, entry: Arc<dyn BlockAtlasEntry>) {
        self.block_atlas.push(entry.clone());
    }
}

#[derive(Clone)]
pub struct BlockTexture {
    src: Image,
    id: Identifier,
    file_name: String
}

impl BlockTexture {
    pub fn new(src: Image, id: Identifier, file_name: impl Into<String>) -> Self {
        Self {
            src, id, file_name: file_name.into()
        }
    }

    pub fn texture_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn src(&self) -> Image {
        self.src.clone()
    }

    pub fn id(&self) -> Identifier {
        self.id.clone()
    }
}

impl Buildable for AllBlockAtlasEntry {}
impl Buildable for PerFaceBlockAtlasEntry {}