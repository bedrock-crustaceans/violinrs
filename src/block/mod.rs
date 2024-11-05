use self::component::BlockComponent;
use crate::block::permutation::BlockPermutation;
use crate::block::state::BlockState;
use crate::vio::Identifier;
use askama::Template;
use std::sync::Mutex;

pub mod block_registry;
pub mod component;

pub mod permutation;
pub mod state;

#[derive(Clone)]
pub struct Block<'a> {
    pub type_id: Identifier,
    pub components: Vec<&'a dyn BlockComponent>,
    pub permutations: Vec<BlockPermutation<'a>>,
    pub states: Vec<&'a dyn BlockState>,
    pub texture_set: String,
    pub sound: String,
}

impl<'a> Block<'a> {
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

        let mut permutations = String::from("");
        for perm in self.permutations.clone() {
            permutations.push_str(perm.serialize().as_str());
            permutations.push(',');
        }
        permutations.pop();

        let mut states = String::from("");
        for perm in self.states.clone() {
            states.push_str(perm.serialize().as_str());
            states.push(',');
        }
        states.pop();

        BlockTemplate {
            type_id: self.clone().type_id.render(),
            components: components_strings.join("\n"),
            traits: "".to_string(),
            states,
            permutations,
        }
        .render()
        .unwrap()
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
}
