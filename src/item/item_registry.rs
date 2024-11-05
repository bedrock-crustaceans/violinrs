use askama::Template;

use super::Item;

#[derive(Clone)]
pub struct ItemRegistry {
    pub items: Vec<Item>,
    pub item_atlas: Vec<ItemAtlasEntry>,
}

#[derive(Template)]
#[template(
    path = "item_serialization/item_atlas_entry.json.jinja2",
    escape = "none"
)]
struct ItemAtlasEntryTemplate {
    pub id: String,
    pub texture_path: String,
}

#[derive(Clone)]
pub struct ItemAtlasEntry {
    pub id: String,
    pub path: String,
    pub texture_name: String,
}
impl ItemAtlasEntry {
    fn serialize(&self) -> String {
        ItemAtlasEntryTemplate {
            texture_path: format!("textures/items/{}", self.clone().texture_name),
            id: self
                .clone()
                .id
                .chars()
                .map(|x| if x == ':' { '_' } else { x })
                .collect(),
        }
        .render()
        .unwrap()
    }
}

pub fn serialize_item_atlas(atlas: &Vec<ItemAtlasEntry>) -> String {
    let mut atlas_string = String::new();
    for entry in atlas {
        atlas_string.push_str(&entry.serialize());
        atlas_string.push(',');
    }
    atlas_string.pop();
    atlas_string
}

impl ItemRegistry {
    pub fn new() -> Self {
        Self {
            items: vec![],
            item_atlas: vec![],
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item.clone());
    }

    pub fn add_texture(&mut self, entry: ItemAtlasEntry) {
        self.item_atlas.push(entry);
    }
}
