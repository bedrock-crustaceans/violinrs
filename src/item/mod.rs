use std::sync::{Arc, Mutex};

use askama::Template;
use crate::vio::Identifier;

pub mod component;
pub mod item_registry;

#[derive(Clone)]
pub struct Item<'a> {
    pub type_id: Identifier<'a>,
    pub components: Vec<Arc<dyn component::ItemComponent>>,
}
impl<'a> Item<'a> {
    pub fn serialize(&self) -> String {
        let components = self.components.clone();
        let mut components_strings: Vec<String> = vec![];
        for component in components {
            let comp = Mutex::new(component);
            let ser = comp.lock();
            let mut fser = match ser {
                Ok(guard) => guard.serialize(),
                Err(_) => "Error serializing".to_string(),
            };
            fser.push(',');
            components_strings.push(fser);
        }
        components_strings.last_mut().unwrap().pop();
        ItemTemplate {
            id: &self.type_id.render(),
            components: components_strings,
        }
        .render()
        .unwrap()
    }
}

#[derive(Template)]
#[template(path = "item_serialization/item_template.json.jinja2", escape = "none")]
struct ItemTemplate<'a> {
    pub id: &'a str,
    pub components: Vec<String>,
}

#[derive(Template)]
#[template(path = "item_serialization/item_texture.json.jinja2", escape = "none")]
pub struct ItemAtlasTemplate<'a> {
    pub name: &'a String,
    pub contents: String,
}
