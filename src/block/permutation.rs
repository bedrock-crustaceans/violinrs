use crate::block::component::BlockComponent;
use askama::Template;
use std::sync::Mutex;

#[derive(Clone)]
pub struct BlockPermutation<'a> {
    pub(crate) condition: &'a str,
    pub(crate) components: Vec<&'a dyn BlockComponent>,
}

#[derive(Template)]
#[template(path = "block_serialization/permutation.json.jinja2", escape = "none")]
struct BlockPermutationTemplate {
    condition: String,
    components: String,
}

impl BlockPermutation<'_> {
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
        let final_components = components_strings.join("\n").to_string();
        BlockPermutationTemplate {
            condition: self.condition.to_string(),
            components: final_components,
        }
        .render()
        .unwrap()
    }
}
