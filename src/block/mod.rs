use std::fs;
use std::path::PathBuf;
use self::component::BlockComponent;
use crate::block::permutation::BlockPermutation;
use crate::block::state::BlockState;
use crate::vio::{Generatable, Identifier, SemVer};
use askama::Template;
use std::sync::Arc;
use jsonxf::pretty_print;

pub mod block_registry;
pub mod component;

pub mod permutation;
pub mod state;
pub mod utils;

#[derive(Clone)]
pub struct Block {
    type_id: Identifier,
    pub format_version: SemVer,
    pub components: Vec<Arc<dyn BlockComponent>>,
    pub permutations: Vec<BlockPermutation>,
    pub states: Vec<Arc<dyn BlockState>>,
}

impl Block {
    pub fn serialize(&self) -> String {
        let components = self.components.clone();
        let mut components_strings: Vec<String> = vec![];
        for component in components {
            let mut fser = component.serialize();
            fser.push(',');
            components_strings.push(fser);
        }
        components_strings.last_mut().unwrap().pop();

        let mut permutations = String::from("");
        for perm in self.permutations.clone() {
            permutations.push_str(perm.serialize().as_str());
            permutations.push(',');
        }
        permutations.pop();

        let mut states = String::from("");
        for state in self.states.clone() {
            states.push_str(state.serialize().as_str());
            states.push(',');
        }
        states.pop();

        BlockTemplate {
            type_id: self.clone().type_id.render(),
            components: components_strings.join("\n"),
            traits: "".to_string(),
            states,
            permutations,
            format_version: self.format_version.render()
        }
        .render()
        .unwrap()
    }


    pub fn new(type_id: Identifier) -> Self {
        Self {
            type_id,
            states: vec![],
            permutations: vec![],
            components: vec![],
            format_version: SemVer::current()
        }
    }

    pub fn using_components(&mut self, components: Vec<Arc<dyn BlockComponent>>) -> Self {
        let mut sc = self.clone();
        sc.components = components;

        sc
    }

    pub fn using_states(&mut self, states: Vec<Arc<dyn BlockState>>) -> Self {
        let mut sc = self.clone();
        sc.states = states;

        sc
    }

    pub fn using_permutations(&mut self, permutations: Vec<BlockPermutation>) -> Self {
        let mut sc = self.clone();
        sc.permutations = permutations;

        sc
    }

    pub fn using_format_version(&self, format_version: SemVer) -> Self {
        let mut sc = self.clone();
        sc.format_version = format_version;

        sc
    }

    pub fn type_id(&self) -> Identifier {
        self.type_id.clone()
    }
}

impl Generatable for Block {
    fn generate(&self, path_buf: impl Into<PathBuf>) {
        fs::write(path_buf.into(), pretty_print(&self.serialize()).unwrap()).unwrap();
    }
}

#[derive(Template)]
#[template(path = "block_serialization/block.json.jinja2", escape = "none")]
struct BlockTemplate {
    type_id: String,
    components: String,
    traits: String,
    permutations: String,
    states: String,
    format_version: String
}
