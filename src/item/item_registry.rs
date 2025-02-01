use super::Item;
use crate::image::Image;
use crate::vio::ViolaDefault;
use askama::Template;
use viola::ViolaDefault;

#[derive(Clone)]
pub struct ItemRegistry {
    pub items: Vec<Item>,
    pub item_atlas: Vec<ItemTexture>,
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

#[derive(Clone, Default, ViolaDefault)]
pub struct ItemTexture {
    pub id: String,
    pub src: Image,
    pub file_name: String,
}
impl ItemTexture {
    fn serialize(&self) -> String {
        ItemAtlasEntryTemplate {
            texture_path: format!("textures/items/{}", self.clone().file_name),
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

    pub fn new(id: impl Into<String>, file_name: impl Into<String>, src: Image) -> Self {
        Self {
            id: id.into(),
            file_name: file_name.into(),
            src,
        }
    }
}

pub fn serialize_item_atlas(atlas: &Vec<ItemTexture>) -> String {
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

    pub fn add_texture(&mut self, entry: ItemTexture) {
        self.item_atlas.push(entry);
    }
}
