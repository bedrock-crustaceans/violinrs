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
    use crate::item::item_registry::ItemTexture;
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
    use crate::item::component::{ItemAllowOffHandComponent, ItemCustomComponents, ItemHandEquippedComponent, ItemMaxStackValueComponent};

    #[test]
    fn main() {
        let scripts = Some(ScriptData {
            mc_server_ui_version: "1.3.0".to_string(),
            mc_server_version: "1.14.0".to_string(),
            paired_scripts_folder: r"./src-scripts",
        }); // Script Data. Can be set to None if project doesn't use scripts
        let mut pack = Pack::new(
            "Violin RS Tests !".to_string(), // Pack Name
            "Violin RS Tests".to_string(), // Pack Identifier
            "NaKeR".to_string(),          // Pack Author
            "1, 0, 0",                    // Pack Version. Separated with commas
            "Nothing here".to_string(),   // Pack Description
            true,                         // Does the project use scripts
            r"C:\Users\narol\AppData\Local\Packages\Microsoft.MinecraftUWP_8wekyb3d8bbwe\LocalState\games\com.mojang\development_behavior_packs", // Developer BP Folder
            r"C:\Users\narol\AppData\Local\Packages\Microsoft.MinecraftUWP_8wekyb3d8bbwe\LocalState\games\com.mojang\development_resource_packs", // Developer RP Folder
            r"./textures/diamond_sword.png", // Pack Icon
            &scripts,                               // Script Data which we defined earlier
        );

        pack.register_item_texture(ItemTexture::new(
            "violin_amethyst_sword",
            "amethyst_sword",
            Image::new(
                r"./textures/diamond_sword.png"
            ).with_hue_shift(120.0)
        ));

        pack.register_item(
            Item::new(Identifier::new("violin", "amethyst_sword")).with_components(vec![
                ItemDamageComponent::new(5).build(),
                ItemDisplayNameComponent::new("Amethyst Sword\n\nThe power of refraction.").build(),
                ItemIconComponent::new("violin_amethyst_sword").build(),
                ItemHandEquippedComponent::new(true).build(),
                ItemMaxStackValueComponent::new(1).build(),
                ItemAllowOffHandComponent::new(true).build()
                // ItemCustomComponents::new(
                //     vec![
                //         Identifier::new("violin", "amethyst_sword")
                //     ]
                // ).build(),
            ]).using_format_version(
                SemVer::new(1, 21, 20)
            ),
        );

        pack.register_item_texture(ItemTexture::new(
            "violin_emerald_sword",
            "emerald_sword",
            Image::new(
                r"./textures/diamond_sword.png"
            ).with_hue_shift(-45.0)
        ));
        pack.register_item(
            Item::new(Identifier::new("violin", "emerald_sword")).with_components(vec![
                ItemDamageComponent::new(5).build(),
                ItemDisplayNameComponent::new("Emerald Sword\n\nThe power of 'Hmm...'.").build(),
                ItemIconComponent::new("violin_emerald_sword").build(),
                ItemHandEquippedComponent::new(true).build(),
                ItemMaxStackValueComponent::new(1).build(),
                ItemAllowOffHandComponent::new(true).build()
                // ItemCustomComponents::new(
                //     vec![
                //         Identifier::new("violin", "amethyst_sword")
                //     ]
                // ).build(),
            ]).using_format_version(
                SemVer::new(1, 21, 20)
            ),
        );

        pack.generate();
        pack.build_to_dev();
    }
}
