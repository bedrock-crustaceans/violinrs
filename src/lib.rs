pub mod block;
pub mod constant;
pub mod image;
pub mod item;
pub mod logger;
pub mod pack;
pub mod recipe;
pub mod template;
pub mod vio;

#[cfg(test)]
mod tests {
    use crate::image::Image;
    use crate::item::component::{
        ItemAllowOffHandComponent, ItemCustomComponents, ItemHandEquippedComponent,
        ItemMaxStackValueComponent,
    };
    use crate::item::item_registry::ItemTexture;
    use crate::recipe::{FurnaceRecipe, RecipeIO, ShapedRecipe, ShapelessRecipe};
    use crate::vio::{Buildable, Identifier, SemVer};
    use crate::{
        item::{
            component::{ItemDamageComponent, ItemDisplayNameComponent, ItemIconComponent},
            Item,
        },
        pack::{Pack, ScriptData},
    };

    fn register_items(pack: &mut Pack) {
        pack.register_item_texture(ItemTexture::new(
            "violin_amethyst_sword",
            "amethyst_sword",
            Image::new(r"./textures/diamond_sword.png").with_hue_shift(120.0),
        ));

        pack.register_item(
            Item::new(Identifier::new("violin", "amethyst_sword"))
                .with_components(vec![
                    ItemDamageComponent::new(14).build(),
                    ItemDisplayNameComponent::new("Amethyst Sword\n\nThe power of refraction.")
                        .build(),
                    ItemIconComponent::new("violin_amethyst_sword").build(),
                    ItemHandEquippedComponent::new(true).build(),
                    ItemMaxStackValueComponent::new(1).build(),
                    ItemAllowOffHandComponent::new(true).build(), // ItemCustomComponents::new(
                                                                  //     vec![
                                                                  //         Identifier::new("violin", "amethyst_sword")
                                                                  //     ]
                                                                  // ).build(),
                ])
                .using_format_version(SemVer::new(1, 21, 20)),
        );

        pack.register_item_texture(ItemTexture::new(
            "violin_emerald_sword",
            "emerald_sword",
            Image::new(r"./textures/diamond_sword.png").with_hue_shift(-45.0),
        ));
        pack.register_item(
            Item::new(Identifier::new("violin", "emerald_sword"))
                .with_components(vec![
                    ItemDamageComponent::new(5).build(),
                    ItemDisplayNameComponent::new("Emerald Sword\n\nThe power of 'Hmm...'.")
                        .build(),
                    ItemIconComponent::new("violin_emerald_sword").build(),
                    ItemHandEquippedComponent::new(true).build(),
                    ItemMaxStackValueComponent::new(1).build(),
                    ItemAllowOffHandComponent::new(true).build(), // ItemCustomComponents::new(
                                                                  //     vec![
                                                                  //         Identifier::new("violin", "amethyst_sword")
                                                                  //     ]
                                                                  // ).build(),
                ])
                .using_format_version(SemVer::new(1, 21, 20)),
        );
    }

    fn register_recipes(pack: &mut Pack) {
        pack.register_recipe(
            ShapedRecipe::new(
                Identifier::new("violin", "emerald_sword_recipe"),
                RecipeIO::new_typed(Identifier::new("violin", "emerald_sword")).using_count(1),
            )
            .using_ingredients(vec![
                RecipeIO::new_typed(Identifier::new("minecraft", "stick")).using_key('S'),
                RecipeIO::new_typed(Identifier::new("minecraft", "emerald")).using_key('E'),
            ])
            .using_pattern(vec!["E", "E", "S"])
            .using_tags(vec!["crafting_table"])
            .build(),
        );

        pack.register_recipe(
            ShapelessRecipe::new(
                Identifier::new("violin", "amethyst_sword_recipe"),
                RecipeIO::new_typed(Identifier::new("violin", "amethyst_sword")).using_count(1),
            )
            .using_ingredients(vec![
                RecipeIO::new_typed(Identifier::new("violin", "emerald_sword")).using_count(1),
                RecipeIO::new_typed(Identifier::new("minecraft", "amethyst_shard")).using_count(8),
            ])
            .using_tags(vec!["crafting_table"])
            .build(),
        );

        pack.register_recipe(
            FurnaceRecipe::new(
                Identifier::new("violin", "amethyst_sword_to_ab"),
                RecipeIO::new_typed(Identifier::new("violin", "amethyst_sword")),
                Identifier::new("minecraft", "amethyst_block"),
            )
            .using_tags(vec!["furnace"])
            .build(),
        );
    }

    #[test]
    fn main() {
        let mut pack = Pack::new(
            "Violin RS Tests !",
            "Violin RS Tests",
            "NaKeR",
            SemVer::new(1, 0, 0),
            "Nothing here",
            r"C:\Users\narol\AppData\Local\Packages\Microsoft.MinecraftUWP_8wekyb3d8bbwe\LocalState\games\com.mojang\development_behavior_packs", // Developer BP Folder
            r"C:\Users\narol\AppData\Local\Packages\Microsoft.MinecraftUWP_8wekyb3d8bbwe\LocalState\games\com.mojang\development_resource_packs", // Developer RP Folder
            Image::new(r"./textures/diamond_sword.png").with_hue_shift(120.0).upscaled(16),
            ScriptData::new(
                SemVer::new_beta(1, 14, 0),
                SemVer::new(1, 3, 0),
                r"./src-scripts",
            ),
        );

        register_items(&mut pack);
        register_recipes(&mut pack);

        pack.generate();
        // pack.build_to_dev();
    }
}
