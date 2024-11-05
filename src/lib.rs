pub mod block;
pub mod constant;
pub mod item;
pub mod logger;
pub mod pack;
pub mod recipe;
pub mod template;
pub mod vio;

#[cfg(test)]
mod tests {
    use crate::item::item_registry::ItemAtlasEntry;
    use crate::vio::{Buildable, Identifier, SemVer};
    use crate::{
        item::{
            component::{
                ItemDamageComponent, ItemDisplayNameComponent,
                ItemIconComponent,
            },
            Item,
        },
        pack::{Pack, ScriptData},
    };
    use crate::item::component::{ItemCustomComponents};

    #[test]
    fn main() {
        let scripts = Some(ScriptData {
            mc_server_ui_version: "1.2.0-beta".to_string(),
            mc_server_version: "1.11.0-beta".to_string(),
            paired_scripts_folder: r"./src-scripts",
        }); // Script Data. Can be set to None if project doesn't use scripts
        let mut pack = Pack::new(
            "Violet Crystal".to_string(), // Pack Name
            "violet_crystal".to_string(), // Pack Identifier
            "NaKeR".to_string(),          // Pack Author
            "1, 0, 0",                    // Pack Version. Separated with commas
            "Nothing here".to_string(),   // Pack Description
            true,                         // Does the project use scripts
            r"C:\Users\User\AppData\Roaming\.minecraft_bedrock\installations\Latest Release\packageData\development_behavior_packs", // Developer BP Folder
            r"C:\Users\User\AppData\Roaming\.minecraft_bedrock\installations\Latest Release\packageData\development_resource_packs", // Developer RP Folder
            r"C:\Users\User\newgito\bluestone.png", // Pack Icon
            &scripts,                               // Script Data which we defined earlier
        );

        pack.register_item_texture(ItemAtlasEntry {
            id: "test_test".to_string(),
            texture_name: "test_test".to_string(),
            path: r"C:\Users\User\newgito\bluestone.png".to_string(),
        });

        pack.register_item(
            Item::new(Identifier::new("test", "test")).with_components(vec![
                ItemDamageComponent::new(5).build(),
                ItemDisplayNameComponent::new("Test Item").build(),
                ItemIconComponent::new("test_item").build(),
                ItemCustomComponents::new(
                    vec![
                        Identifier::new("hello", "world")
                    ]
                ).build(),
            ]).using_format_version(
                SemVer::new(1, 20, 80)
            ),
        );


        pack.generate(Some(false));
        pack.build_to_dev();
    }
}
