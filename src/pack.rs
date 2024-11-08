use crate::item::item_registry::{serialize_item_atlas, ItemTexture};
use crate::item::ItemAtlasTemplate;
use crate::item::{item_registry::ItemRegistry, Item};
use crate::logger::info;
use crate::recipe::Recipe;
use crate::template::{BpManifestTemplate, RpManifestTemplate};
use askama::Template;
use fs_extra::dir;
use fs_extra::dir::DirEntryAttr::Path;
use std::fs;
use std::path::PathBuf;
use std::string::ToString;
use std::sync::Arc;
use serde_json::Value;
use crate::block::block_registry::{
    serialize_block_atlas, serialize_terrain_atlas, BlockAtlasTemplate, BlockRegistry,
    TerrainAtlasTemplate,
};
use crate::block::Block;
use crate::image::Image;
use crate::vio::SemVer;
use uuid::Uuid;

const RESULT_FOLDER: &str = "violin_output";

#[derive(Clone)]
pub struct ScriptData {
    pub mc_server_version: SemVer,
    pub mc_server_ui_version: SemVer,
    pub paired_scripts_folder: String,
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
        })
    }
}

#[derive(Clone)]
pub struct Pack<'a> {
    pub name: String,
    pub id: String,
    pub author: String,
    pub version: SemVer,
    pub description: String,
    pub scripts: Option<ScriptData>,
    pub dev_bp_folder: String,
    pub dev_rp_folder: String,
    pub icon: Image,
    pub item_registry: ItemRegistry,
    pub recipes: Vec<Arc<dyn Recipe>>,
    pub block_registry: BlockRegistry<'a>,
}

impl<'a> Pack<'a> {
    pub fn new(
        name: impl Into<String> + Clone,
        id: impl Into<String> + Clone,
        author: impl Into<String> + Clone,
        version: SemVer,
        description: impl Into<String> + Clone,
        dev_bp_folder: impl Into<String> + Clone,
        dev_rp_folder: impl Into<String> + Clone,
        icon: Image,
        scripts: Option<ScriptData>,
    ) -> Self {
        info(
            format!(
                "Registering Pack \"{}\"(\"{}\")",
                name.clone().into(),
                id.clone().into()
            ),
            "[ PACK ]".to_string(),
        );
        let items: ItemRegistry = ItemRegistry::new();
        let pack = Self {
            name: name.into(),
            id: id.into(),
            author: author.into(),
            version,
            description: description.into(),
            scripts,
            dev_bp_folder: dev_bp_folder.into(),
            dev_rp_folder: dev_rp_folder.into(),
            icon,
            item_registry: items.clone(),
            recipes: Vec::new(),
            block_registry: BlockRegistry::new(),
        };
        pack
    }

    pub fn generate(&mut self) -> () {
        info(
            format!("Creating Pack \"{}\"(\"{}\")", &self.name, &self.id),
            "[ PACK ]".to_string(),
        );

        let mut bp_uuid_1 = Uuid::new_v4().to_string();
        let mut bp_uuid_2 = Uuid::new_v4().to_string();
        let mut bp_uuid_3 = Uuid::new_v4().to_string();
        let mut rp_uuid_1 = Uuid::new_v4().to_string();
        let mut rp_uuid_2 = Uuid::new_v4().to_string();

        if fs::exists(format!(
            "./violin_output/packs/{}/RP/manifest.json",
            &self.id
        ))
            .unwrap_or(false) {
            let v: Value = serde_json::from_str(fs::read_to_string(format!(
                "./{RESULT_FOLDER}/packs/{}/RP/manifest.json",
                &self.id
            )).unwrap().as_str()).unwrap();

            rp_uuid_1 = v["header"]["uuid"].as_str().unwrap().to_string();
            rp_uuid_2 = v["modules"][0]["uuid"].as_str().unwrap().to_string();
        }

        if fs::exists(format!(
            "./{RESULT_FOLDER}/packs/{}/BP/manifest.json",
            &self.id
        ))
            .unwrap_or(false) {
            let v: Value = serde_json::from_str(fs::read_to_string(format!(
                "./{RESULT_FOLDER}/packs/{}/BP/manifest.json",
                &self.id
            )).unwrap().as_str()).unwrap();

            bp_uuid_1 = v["header"]["uuid"].as_str().unwrap().to_string();
            bp_uuid_2 = v["modules"][0]["uuid"].as_str().unwrap().to_string();
            if self.scripts.is_some() {
                bp_uuid_3 = v["modules"][1]["uuid"].as_str().unwrap_or("NULL").to_string();
            }
        }


        if let Ok(content) = fs::read_dir(format!("./{RESULT_FOLDER}/packs/{}/BP", &self.id)) {
            let mut files =
                fs_extra::dir::get_dir_content(format!("./{RESULT_FOLDER}/packs/{}/BP", &self.id))
                    .unwrap();
            for file in files.files.iter() {
                // if file.ends_with("manifest.json") {
                //     continue;
                // }
                fs::remove_file(file).expect("Cannot remove file");
            }
        }
        if let Ok(content) = fs::read_dir(format!("./{RESULT_FOLDER}/packs/{}/RP", &self.id)) {
            let mut files =
                fs_extra::dir::get_dir_content(format!("./{RESULT_FOLDER}/packs/{}/RP", &self.id))
                    .unwrap();
            for file in files.files.iter() {
                if file.ends_with("manifest.json") {
                    continue;
                }
                fs::remove_file(file).expect("Cannot remove file");
            }
        }

        let _ = fs::create_dir_all(format!("./{RESULT_FOLDER}/packs/{}/BP", &self.id));
        let _ = fs::create_dir_all(format!("./{RESULT_FOLDER}/packs/{}/RP", &self.id));

        let bp_manifest: String = BpManifestTemplate {
            name: &self.name.as_str(),
            author: &self.author.as_str(),
            description: &self.description.as_str(),
            use_scripts: &self.scripts.is_some(),
            uuid_1: bp_uuid_1.clone().as_str(),
            uuid_2: bp_uuid_2.clone().as_str(),
            uuid_3: bp_uuid_3.clone().as_str(),
            server_ui_version: match &self.scripts {
                Some(scripts) => scripts.mc_server_ui_version.clone().render(),
                None => "0.0.0".to_string(),
            },
            server_version: match &self.scripts {
                Some(scripts) => scripts.mc_server_version.clone().render(),
                None => "0.0.0".to_string(),
            },
            version: &self.version.render_commas(),
        }
        .render()
        .unwrap();
        match fs::write(
            format!("./{RESULT_FOLDER}/packs/{}/BP/manifest.json", &self.id),
            bp_manifest,
        ) {
            Ok(_) => (),
            Err(_) => (),
        };

        let rp_manifest: String = RpManifestTemplate {
            name: &self.name.as_str(),
            author: &self.author.as_str(),
            description: &self.description.as_str(),
            uuid_1: rp_uuid_1.as_str(),
            uuid_2: rp_uuid_2.as_str(),
            version: &self.version.render_commas(),
        }
        .render()
        .unwrap();
        match fs::write(
            format!("./{RESULT_FOLDER}/packs/{}/RP/manifest.json", &self.id),
            rp_manifest,
        ) {
            Ok(_) => (),
            Err(_) => (),
        };

        // let _ = match fs::write(
        //     format!(
        //         "./{RESULT_FOLDER}/packs/{}/IMPORTANT.MD",
        //         &self.id
        //     ),
        //     crate::constant::important::IMPORTANTMD,
        // ) {
        //     Ok(_) => info("VioletCrystal has generated an IMPORTANT.MD file. Consider checking it before the next run".to_string(), "[ IMPORTANT ]".to_string()),
        //     Err(_) => (),
        // };

        self.icon.build(PathBuf::from(format!(
            "./{RESULT_FOLDER}/packs/{}/BP/pack_icon.png",
            &self.id
        )));
        self.icon.build(PathBuf::from(format!(
            "./{RESULT_FOLDER}/packs/{}/RP/pack_icon.png",
            &self.id
        )));

        if self.scripts.is_some() {
            self.pair_scripts();
        }

        self.generate_items();
        self.generate_blocks();
        self.generate_recipes();
    }

    pub fn register_recipe<'b>(&mut self, recipe: Arc<dyn Recipe>) {
        self.recipes.push(recipe.clone());
        info(
            format!("Registering recipe {}", recipe.id().render()),
            "[ RECIPE ]".to_string(),
        );
    }

    pub fn register_item(&mut self, item: Item) {
        self.item_registry.add_item(item.clone());
        info(
            format!("Registering Item \"{}\"", &item.type_id().render()),
            "[ ITEM ]".to_string(),
        );
    }

    pub fn register_item_texture(&mut self, texture: ItemTexture) {
        info(
            format!(
                "Registering Item Texture \"{}\"",
                texture.clone().texture_name
            ),
            "[ ITEM ][ TEXTURE ]".to_string(),
        );
        self.item_registry.add_texture(texture);
    }

    fn generate_items(&mut self) {
        let _ = fs::create_dir_all(format!("./{RESULT_FOLDER}/packs/{}/BP/items/", &self.id));
        let _ = fs::create_dir_all(format!("./{RESULT_FOLDER}/packs/{}/RP/textures/", &self.id));

        let itreg = self.item_registry.clone();

        for item in itreg.items {
            extern crate jsonxf;
            info(
                format!("Generating Item \"{}\"", &item.type_id().render()),
                "[ ITEM ]".to_string(),
            );
            let file_name: String = item
                .type_id()
                .render()
                .chars()
                .into_iter()
                .map(|el| if el == ':' { '_' } else { el })
                .collect();
            let content = item.serialize();
            let pretty_content = jsonxf::pretty_print(&content).unwrap();
            let _ = match fs::write(
                format!(
                    "./{RESULT_FOLDER}/packs/{}/BP/items/{}.item.json",
                    &self.id, &file_name
                ),
                pretty_content,
            ) {
                Ok(_) => "Ok!",
                Err(_) => "Err!",
            };
            let _ = fs::create_dir_all(format!(
                "./{RESULT_FOLDER}/packs/{}/RP/textures/items",
                &self.id
            ));
        }

        self.generate_item_atlas();
    }

    fn generate_item_atlas(&self) {
        let _ = fs::create_dir_all(format!(
            "./{RESULT_FOLDER}/packs/{}/RP/textures/items",
            &self.id
        ));
        let content_raw = ItemAtlasTemplate {
            name: &self.name,
            contents: serialize_item_atlas(&self.item_registry.item_atlas),
        }
        .render()
        .unwrap();

        for entry in &self.item_registry.item_atlas {
            let file_name: String = entry.clone().texture_name;

            entry.src.build(PathBuf::from(format!(
                "./{RESULT_FOLDER}/packs/{}/RP/textures/items/{}.png",
                &self.id, &file_name
            )))
        }
        let content = jsonxf::pretty_print(content_raw.as_str()).unwrap();
        let _ = match fs::write(
            format!(
                "./{RESULT_FOLDER}/packs/{}/RP/textures/item_texture.json",
                &self.id
            ),
            content,
        ) {
            Ok(_) => "Ok!",
            Err(_) => "Err!",
        };
    }

    pub fn build_to_dev(&self) {
        let _ = fs::remove_dir_all(format!("{}/{}_BP", &self.dev_bp_folder, &self.id));
        let _ = fs::remove_dir_all(format!("{}/{}_RP", &self.dev_rp_folder, &self.id));

        let _ = fs::create_dir_all(format!("{}/{}_BP", &self.dev_bp_folder, &self.id));
        let _ = fs::create_dir_all(format!("{}/{}_RP", &self.dev_rp_folder, &self.id));

        info(
            format!("Copying {}'s BP to DevBPFolder", &self.id),
            "[ PACK ]".to_string(),
        );
        let _ = fs_extra::dir::copy(
            format!("./{RESULT_FOLDER}/packs/{}/BP/", &self.id),
            format!("{}/{}_BP/", &self.dev_bp_folder, &self.id),
            &dir::CopyOptions::new().content_only(true).overwrite(true),
        );
        info(
            format!("Copying {}'s RP to DevRPFolder", &self.id),
            "[ PACK ]".to_string(),
        );
        let _ = fs_extra::dir::copy(
            format!("./{RESULT_FOLDER}/packs/{}/RP/", &self.id),
            format!("{}/{}_RP/", &self.dev_rp_folder, &self.id),
            &dir::CopyOptions::new().content_only(true).overwrite(true),
        );
    }

    pub fn pair_scripts(&self) {
        let _ = fs::create_dir_all(format!("./{RESULT_FOLDER}/packs/{}/BP/scripts/", &self.id));
        // let path = self.scripts.as_ref().unwrap().paired_scripts_folder;
        // let _ = fs_extra::dir::copy(
        //     path,
        //     format!("./{RESULT_FOLDER}/packs/{}/BP/scripts/", &self.id),
        //     &fs_extra::dir::CopyOptions::new()
        //         .overwrite(true)
        //         .content_only(true),
        // );
        // TODO
        // info(
        //     format!("Paired scripts from folder {}", path),
        //     "[ SCRIPTS ]".to_string(),
        // )
    }

    fn generate_recipes(&self) {
        let _ = fs::create_dir_all(format!("./{RESULT_FOLDER}/packs/{}/BP/recipes/", &self.id));
        let iterator: Vec<Arc<dyn Recipe>> = self.recipes.clone();
        for recipe in iterator {
            info(
                format!("Generating Recipe \"{}\"", recipe.id().render()),
                "[ RECIPE ]".to_string(),
            );
            let file_name: String = recipe
                .id()
                .render()
                .chars()
                .into_iter()
                .map(|el| if el == ':' { '_' } else { el })
                .collect();
            let content = recipe.serialize();
            let pretty_content = jsonxf::pretty_print(&content).unwrap();
            let _ = match fs::write(
                format!(
                    "./{RESULT_FOLDER}/packs/{}/BP/recipes/{}.recipe.json",
                    &self.id, &file_name
                ),
                pretty_content,
            ) {
                Ok(_) => "Ok!",
                Err(_) => "Err!",
            };
        }
    }

    fn generate_blocks(&mut self) {
        let _ = fs::create_dir_all(format!("./{RESULT_FOLDER}/packs/{}/BP/blocks/", &self.id));
        let _ = fs::create_dir_all(format!("./{RESULT_FOLDER}/packs/{}/RP/textures/", &self.id));

        for block in self.block_registry.blocks.iter() {
            extern crate jsonxf;
            info(
                format!("Generating Block \"{}\"", &block.type_id.render()),
                "[ BLOCK ]".to_string(),
            );
            let file_name: String = block
                .type_id
                .render()
                .chars()
                .into_iter()
                .map(|el| if el == ':' { '_' } else { el })
                .collect();
            let content = block.serialize();
            let pretty_content = jsonxf::pretty_print(&content).unwrap();
            let _ = match fs::write(
                format!(
                    "./{RESULT_FOLDER}/packs/{}/BP/blocks/{}.block.json",
                    &self.id, &file_name
                ),
                pretty_content,
            ) {
                Ok(_) => "Ok!",
                Err(_) => "Err!",
            };
            let _ = fs::create_dir_all(format!(
                "./{RESULT_FOLDER}/packs/{}/RP/textures/blocks",
                &self.id
            ));
            let _ = match fs::copy(
                block.clone().texture_set,
                format!(
                    "./{RESULT_FOLDER}/packs/{}/RP/textures/blocks/{}.png",
                    &self.id, &file_name
                ),
            ) {
                Ok(_) => "Ok!",
                Err(_) => "Err!",
            };
        }

        self.generate_block_atlas();
        self.generate_terrain_atlas();
    }

    fn generate_block_atlas(&self) {
        let _ = fs::create_dir_all(format!(
            "./{RESULT_FOLDER}/packs/{}/RP/textures/blocks",
            &self.id
        ));
        let content_raw = BlockAtlasTemplate {
            content: serialize_block_atlas(&self.block_registry.block_atlas),
        }
        .render()
        .unwrap();
        let content = jsonxf::pretty_print(content_raw.as_str()).unwrap();
        let _ = match fs::write(
            format!("./{RESULT_FOLDER}/packs/{}/RP/blocks.json", &self.id),
            content,
        ) {
            Ok(_) => "Ok!",
            Err(_) => "Err!",
        };
    }

    fn generate_terrain_atlas(&self) {
        let _ = fs::create_dir_all(format!("./{RESULT_FOLDER}/packs/{}/RP/textures/", &self.id));
        let content_raw = TerrainAtlasTemplate {
            content: serialize_terrain_atlas(&self.block_registry.terrain_atlas),
            pack_name: self.name.clone(),
        }
        .render()
        .unwrap();
        let content = jsonxf::pretty_print(content_raw.as_str()).unwrap();
        let _ = match fs::write(
            format!(
                "./{RESULT_FOLDER}/packs/{}/RP/textures/terrain_texture.json",
                &self.id
            ),
            content,
        ) {
            Ok(_) => "Ok!",
            Err(_) => "Err!",
        };
    }

    pub fn register_block(&mut self, block: Block<'a>) {
        self.block_registry.add_block(block.clone());
        info(
            format!("Registering block {}", block.type_id.render()),
            "[ BLOCK ]".to_string(),
        );
    }
}
