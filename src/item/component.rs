use crate::vio::{Buildable, Identifier, RangeDescriptor};
use serde::{Deserialize, Serialize};
use item_component_macros::item_component;
use crate::block::utils::BlockDestroySpeed;
use crate::item::utils::EnchantableSlot;

pub trait ItemComponent {
    fn serialize(&self) -> String;
}

// * ItemDamageComponent

item_component! {
    name = Damage for "minecraft:damage";
    value has i32 for "value" with "public" "optional";
}

// * ItemDisplayNameComponent

item_component! {
    name = DisplayName for "minecraft:display_name";
    value has String for "value" with "public" "into";
}

// * ItemIconComponent

item_component! {
    name = Icon for "minecraft:icon";
    texture has String for "texture" with "public" "into";
}

// * ItemFuelComponent

item_component! {
    name = Fuel for "minecraft:fuel";
    duration has i32 for "duration" with "public";
}

// * ItemHandEquippedComponent

item_component! {
    name = HandEquipped for "minecraft:hand_equipped";
    value has bool for "value" with "public";
}

// * ItemAllowOffHandComponent

item_component! {
    name = AllowOffHand for "minecraft:allow_off_hand";
    value has bool for "value" with "public";
}

// * ItemMaxStackSizeComponent

item_component! {
    name = MaxStackSize for "minecraft:max_stack_size";
    value has i32 for "value" with "public";
}

// * ItemDurabilityComponent
// #[derive(Template)]
// #[template(
//     path = "item_serialization/components/durability.json.jinja2",
//     escape = "none"
// )]
// struct ItemDurabilityComponentTemplate {
//     min_chance: i32,
//     max_chance: i32,
//     durability: i32,
// }
// #[derive(Clone)]
// pub struct ItemDurabilityComponent {
//     min_chance: i32,
//     max_chance: i32,
//     durability: i32,
// }
//
// impl Buildable for ItemDurabilityComponent {}
//
// impl ItemComponent for ItemDurabilityComponent {
//     fn serialize(&self) -> String {
//         let value = self.durability;
//         let min_c = self.min_chance;
//         let max_c = self.max_chance;
//         let val: String = ItemDurabilityComponentTemplate {
//             max_chance: max_c,
//             min_chance: min_c,
//             durability: value,
//         }
//         .render()
//         .unwrap();
//         val
//     }
// }
//
// impl ItemDurabilityComponent {
//     pub fn new(min_chance: i32, max_chance: i32, durability: i32) -> Self {
//         Self {
//             min_chance,
//             max_chance,
//             durability,
//         }
//     }
//
//     pub fn min_chance(&self) -> i32 {
//         self.min_chance
//     }
//
//     pub fn max_chance(&self) -> i32 {
//         self.max_chance
//     }
//
//     pub fn durability(&self) -> i32 {
//         self.durability
//     }
// }

item_component! {
    name = Durabilty for "minecraft:durability";
    damage_chance has RangeDescriptor<i32> for "damage_chance" with "public";
    max_durability has i32 for "max_durability" with "public";
}

// * ItemArmorComponent

item_component! {
    name = Armor for "minecraft:armor";
    protection has i32 for "protection" with "public";
}

// * ItemCreativeCategoryComponent

item_component! {
    name = CreativeCategory for "minecraft:creative_category";
    parent has String for "parent" with "public" "into";
}

// * ItemRepairableComponent

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ItemRepairEntry {
    pub items: Vec<String>,
    pub repair_amount: String,
}
impl ItemRepairEntry {
    pub fn new(items: Vec<impl Into<String> + Clone>, amount: impl Into<String>) -> Self {
        Self {
            items: items.iter().map(|x| (*x).clone().into()).collect(),
            repair_amount: amount.into(),
        }
    }
}

item_component! {
    name = Repairable for "minecraft:repairable";
    repair_items has Vec<ItemRepairEntry> for "repair_items" with "public";
}

// * ItemCustomComponents

item_component! {
    name = CustomComponents for "minecraft:custom_components" with "transparency";
    components has Vec<Identifier> for "minecraft:custom_components" with "public";
}

// * BundleInteraction

item_component! {
    name = BundleInteraction for "minecraft:bundle_interaction";
    viewable_slots has u8 for "num_viewable_slots" with "public";
}

// * CanDestroyInCreative

item_component! {
    name = CanDestroyInCreative for "minecraft:can_destroy_in_creative";
    value has bool for "value" with "public";
}

// * Cooldown

item_component! {
    name = Cooldown for "minecraft:cooldown";
    category has String for "category" with "public";
    duration has f64 for "duration" with "public";
}

// * DamageAbsorption

item_component! {
    name = DamageAbsorption for "minecraft:damage_absorption";
    absorbable_causes has Vec<String> for "absorbable_causes" with "public";
}

// * Digger

item_component! {
    name = Digger for "minecraft:digger";
    use_efficiency has bool for "use_efficiency" with "public";
    destroy_speeds has Vec<BlockDestroySpeed> for "destroy_speeds" with "public";
}

// * Enchantable

item_component! {
    name = Enchantable for "minecraft:enchantable";
    value has u8 for "value" with "public";
    slot has EnchantableSlot for "slot" with "public";
}

// * EntityPlacer

item_component! {
    name = EntityPlacer for "minecraft:entity_placer";
    entity has Identifier for "entity" with "public";
    dispense_on has Vec<Identifier> for "dispense_on" with "public";
    use_on has Vec<Identifier> for "use_on" with "public";
}