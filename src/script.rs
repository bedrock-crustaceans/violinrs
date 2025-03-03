use std::sync::{Arc, RwLock};
use crate::pack::Pack;
use crate::vio::SemVer;

#[derive(Clone)]
pub struct ScriptData {
    pub mc_server_version: SemVer,
    pub mc_server_ui_version: SemVer,
    pub paired_scripts_folder: String,
    pub additions: Vec<Arc<dyn ScriptAddition>>
}

impl ScriptData {
    pub fn new(
        mc_server_version: SemVer,
        mc_server_ui_version: SemVer,
        paired_scripts_folder: impl Into<String>,
    ) -> Option<Self> {
        Some(Self {
            mc_server_version,
            mc_server_ui_version,
            paired_scripts_folder: paired_scripts_folder.into(),
            additions: vec![],
        })
    }

    pub fn generate_additions(&self, pack: Arc<RwLock<&&Pack>>) {
        for addition in &self.additions {
            addition.build_addition(pack.clone())
        }
    }
}

pub trait ScriptAddition {
    fn build_addition(&self, pack: Arc<RwLock<&&Pack>>);
}