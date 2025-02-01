use crate::vio::ViolaDefault;
use std::sync::Arc;

use crate::vio::{Identifier, SemVer};
use askama::Template;
use viola::ViolaDefault;

pub mod component;
pub mod item_registry;
pub mod utils;

#[derive(Clone, Default, ViolaDefault)]
pub struct Item {
    pub(crate) type_id: Identifier,
    pub format_version: SemVer,
    pub components: Vec<Arc<dyn component::ItemComponent>>,
}

impl Item {
    pub fn serialize(&self) -> String {
        let components = self.components.clone();
        let mut components_strings: Vec<String> = vec![];
        for component in components {
            let mut fser = component.serialize();
            fser.push(',');
            components_strings.push(fser);
        }
        components_strings.last_mut().unwrap().pop();
        ItemTemplate {
            id: &self.type_id.render(),
            components: components_strings,
            format_version: self.format_version.render(),
        }
        .render()
        .unwrap()
    }

    pub fn new(type_id: Identifier) -> Self {
        Item {
            type_id,
            components: vec![],
            format_version: SemVer::current(),
        }
    }

    pub fn with_component(&mut self, component: Arc<dyn component::ItemComponent>) -> Self {
        self.components.push(component);

        self.clone()
    }

    pub fn using_components(&mut self, components: Vec<Arc<dyn component::ItemComponent>>) -> Self {
        let mut sc = self.clone();
        sc.components = components;

        sc
    }

    pub fn type_id(&self) -> Identifier {
        self.type_id.clone()
    }

    pub fn using_type_id(&self, type_id: Identifier) -> Self {
        let mut cloned_self = self.clone();

        cloned_self.type_id = type_id;

        cloned_self
    }

    pub fn using_format_version(&self, format_version: SemVer) -> Self {
        let mut cloned_self = self.clone();

        cloned_self.format_version = format_version;

        cloned_self
    }
}

#[derive(Template)]
#[template(path = "item_serialization/item_template.json.jinja2", escape = "none")]
struct ItemTemplate<'a> {
    pub id: &'a str,
    pub components: Vec<String>,
    pub format_version: String,
}

#[derive(Template)]
#[template(path = "item_serialization/item_texture.json.jinja2", escape = "none")]
pub struct ItemAtlasTemplate<'a> {
    pub name: &'a String,
    pub contents: String,
}
