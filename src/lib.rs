pub mod block;
pub mod constant;
pub mod item;
pub mod logger;
pub mod pack;
pub mod recipe;
pub mod template;
pub mod vio;
pub mod image;

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
    use crate::image::Image;
    use crate::item::component::ItemCustomComponents;

    #[test]
    fn main() {
        let scripts = Some(ScriptData {
            mc_server_ui_version: "1.2.0-beta".to_string(),
            mc_server_version: "1.11.0-beta".to_string(),
            paired_scripts_folder: r"./src-scripts",
        }); // Script Data. Can be set to None if project doesn't use scripts
        let mut pack = Pack::new(
            "Violin RS Tests".to_string(), // Pack Name
            "Violin RS".to_string(), // Pack Identifier
            "NaKeR".to_string(),          // Pack Author
            "1, 0, 0",                    // Pack Version. Separated with commas
            "Nothing here".to_string(),   // Pack Description
            true,                         // Does the project use scripts
            r"C:\Users\narol\AppData\Local\Packages\Microsoft.MinecraftUWP_8wekyb3d8bbwe\LocalState\games\com.mojang\development_behavior_packs", // Developer BP Folder
            r"C:\Users\narol\AppData\Local\Packages\Microsoft.MinecraftUWP_8wekyb3d8bbwe\LocalState\games\com.mojang\development_resource_packs", // Developer RP Folder
            r"C:\Users\User\newgito\bluestone.png", // Pack Icon
            &None,                               // Script Data which we defined earlier
        );

        pack.register_item_texture(ItemAtlasEntry::new(
            "violin_amethyst_sword",
            "amethyst_sword",
            Image::new(
                r"./textures/diamond_sword.png"
            ).with_hue_shift(120.0)
        ));

        pack.register_item(
            Item::new(Identifier::new("violin", "amethyst_sword")).with_components(vec![
                ItemDamageComponent::new(5).build(),
                ItemDisplayNameComponent::new("Amethyst Sword").build(),
                ItemIconComponent::new("violin_amethyst_sword").build(),
                ItemCustomComponents::new(
                    vec![
                        Identifier::new("violin", "amethyst_sword")
                    ]
                ).build(),
            ]).using_format_version(
                SemVer::new(1, 21, 40)
            ),
        );


        pack.generate(false);
        pack.build_to_dev();
    }
}
