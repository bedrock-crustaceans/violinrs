pub mod block;
pub mod constant;
pub mod image;
pub mod item;
pub mod localization;
pub mod logger;
pub mod pack;
pub mod recipe;
pub mod template;
pub mod vio;

#[cfg(test)]
mod tests {
    use crate::item::component::{ItemDamageAbsorptionComponent, ItemDiggerComponent, ItemDurabiltyComponent, ItemRepairEntry, ItemRepairableComponent};
use crate::block::block_registry::{
        BlockRegistry, BlockTexture, Faces,
        PerFaceBlockAtlasEntry,
    };
    use crate::block::component::{
        BlockDisplayNameComponent, BlockFrictionComponent,
    };
    use crate::block::state::{NumericBlockState};
    use crate::block::Block;
    use crate::image::{Image};
    use crate::item::component::{
        ItemAllowOffHandComponent, ItemCustomComponentsComponent, ItemHandEquippedComponent,
        ItemMaxStackSizeComponent,
    };
    use crate::item::item_registry::ItemTexture;
    use crate::localization::Localization;
    use crate::recipe::{FurnaceRecipe, RecipeIO, ShapedRecipe, ShapelessRecipe};
    use crate::vio::{vec_into, Buildable, Generatable, Identifier, MolangStatement, RangeDescriptor, SemVer};
    use crate::{
        item::{
            component::{ItemDamageComponent, ItemDisplayNameComponent, ItemIconComponent},
            Item,
        },
        pack::{Pack, ScriptData},
    };
    use crate::block::utils::{BlockDescriptor, BlockDestroySpeed};

    fn register_items(pack: &mut Pack) {
        pack.register_item_texture(ItemTexture::new(
            "violin_amethyst_sword",
            "amethyst_sword",
            Image::new(r"./textures/diamond_sword.png").with_hue_shift(120.0),
        ));

        pack.register_item(
            Item::new(Identifier::new("violin", "amethyst_sword"))
                .using_components(vec![
                    ItemDamageComponent::new(14).build(),
                    ItemDisplayNameComponent::new("Amethyst Sword\n\nThe power of refraction.")
                        .build(),
                    ItemIconComponent::new("violin_amethyst_sword").build(),
                    ItemHandEquippedComponent::new(true).build(),
                    ItemMaxStackSizeComponent::new(1).build(),
                    ItemAllowOffHandComponent::new(true).build(),
                    ItemCustomComponentsComponent::new(vec![Identifier::new("vio", "amethyst_sword")])
                        .build(),
                    ItemDurabiltyComponent::new(
                        RangeDescriptor::new(0, 1),
                        1280
                    ).build(),
                    ItemRepairableComponent::new(
                        vec![
                            ItemRepairEntry::new(vec!["minecraft:stick"], "(1.0)")
                        ]
                    ).build(),
                    ItemDamageAbsorptionComponent::new(
                        vec_into(vec![
                            "entity_attack"
                        ])
                    ).build(),
                    ItemDiggerComponent::new(
                        false,
                        vec![
                            BlockDestroySpeed::new(
                                BlockDescriptor::new(
                                    MolangStatement::new("q.any_tag('hello_world', 'hello_violin')")
                                ),
                                10
                            )
                        ]
                    ).build(),

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
                .using_components(vec![
                    ItemDamageComponent::new(5).build(),
                    ItemDisplayNameComponent::new("Emerald Sword\n\nThe power of 'Hmm...'.")
                        .build(),
                    ItemIconComponent::new("violin_emerald_sword").build(),
                    ItemHandEquippedComponent::new(true).build(),
                    ItemMaxStackSizeComponent::new(1).build(),
                    ItemAllowOffHandComponent::new(true).build(), // ItemCustomComponentsComponent::new(
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
            "Light-Elytra Booster",
            "light-elytra-booster",
            "NaKeR",
            SemVer::new(1, 0, 0),
            "Official add-on made using Violin.rs",
            r"C:\Users\narol\AppData\Local\Packages\Microsoft.MinecraftUWP_8wekyb3d8bbwe\LocalState\games\com.mojang\development_behavior_packs", // Developer BP Folder
            r"C:\Users\narol\AppData\Local\Packages\Microsoft.MinecraftUWP_8wekyb3d8bbwe\LocalState\games\com.mojang\development_resource_packs", // Developer RP Folder
            Image::new(r"./textures/diamond_sword.png")
                .with_hue_shift(120.0)
                .upscaled(16),
            ScriptData::new(
                SemVer::new(1, 14, 0),
                SemVer::new(1, 3, 0),
                r"./src-scripts",
            ),
        );

        register_items(&mut pack);
        register_recipes(&mut pack);

        pack.register_block(
            Block::new(Identifier::new("violin", "test"))
                .using_components(vec![BlockFrictionComponent::new(0.1).build()])
                .using_states(vec![NumericBlockState::new(
                    Identifier::new("v", "t"),
                    vec![0, 1, 2, 3],
                )
                .build()])
                .using_format_version(SemVer::new(1, 21, 0)),
        );

        pack.register_block_texture(BlockTexture::new(
            Image::new("./textures/diamond_sword.png"),
            Identifier::new("violin", "test"),
            "violin-tex-test",
        ));

        pack.register_block_texture(BlockTexture::new(
            Image::new("./textures/diamond_sword.png").with_hue_shift(60.0),
            Identifier::new("violin", "test_up"),
            "violin-tex-test-up",
        ));

        pack.register_block_atlas_entry(
            PerFaceBlockAtlasEntry::new(
                Identifier::new("violin", "test"),
                Faces::new_identifiers(
                    Identifier::new("violin", "test_up"),
                    Identifier::new("violin", "test"),
                    Identifier::new("violin", "test"),
                    Identifier::new("violin", "test"),
                    Identifier::new("violin", "test"),
                    Identifier::new("violin", "test"),
                ),
                "stone",
            )
            .build(),
        );

        let mut en_us = Localization::new("en_US");

        en_us.add_block_name(Identifier::new("violin", "test"), "Violin Test");
        pack.add_localization(en_us);

        pack.generate();
        pack.build_to_dev();
    }

    #[test]
    fn standalone() {
        let mut block_reg = BlockRegistry {
            block_atlas: vec![],
            blocks: vec![],
            terrain_atlas: vec![],
            textures: vec![],
        };

        let block = Block::new(Identifier::new("hello", "world"))
            .using_components(vec![BlockDisplayNameComponent::new("Hello, world!").build()])
            .using_format_version(SemVer::new(1, 21, 40));

        block_reg.add_block(block.clone());

        block.generate("./standalone_gen/block.json");

        // item_component! {
        //     name = IDK for "minecraft:idk";
        //     num has String for "num" with into;
        // }
    }
}
