use crate::vio::{Buildable, Identifier};
use askama::Template;
use item_component_macros::item_component;

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
#[derive(Template)]
#[template(
    path = "item_serialization/components/durability.json.jinja2",
    escape = "none"
)]
struct ItemDurabilityComponentTemplate {
    min_chance: i32,
    max_chance: i32,
    durability: i32,
}
#[derive(Clone)]
pub struct ItemDurabilityComponent {
    min_chance: i32,
    max_chance: i32,
    durability: i32,
}

impl Buildable for ItemDurabilityComponent {}

impl ItemComponent for ItemDurabilityComponent {
    fn serialize(&self) -> String {
        let value = self.durability;
        let min_c = self.min_chance;
        let max_c = self.max_chance;
        let val: String = ItemDurabilityComponentTemplate {
            max_chance: max_c,
            min_chance: min_c,
            durability: value,
        }
        .render()
        .unwrap();
        val
    }
}

impl ItemDurabilityComponent {
    pub fn new(min_chance: i32, max_chance: i32, durability: i32) -> Self {
        Self {
            min_chance,
            max_chance,
            durability,
        }
    }

    pub fn min_chance(&self) -> i32 {
        self.min_chance
    }

    pub fn max_chance(&self) -> i32 {
        self.max_chance
    }

    pub fn durability(&self) -> i32 {
        self.durability
    }
}

// * ItemArmorComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/armor.json.jinja2",
    escape = "none"
)]
struct ItemArmorComponentTemplate {
    protection: i32,
}
#[derive(Clone)]
pub struct ItemArmorComponent {
    protection: i32,
}

impl ItemArmorComponent {
    pub fn new(protection: i32) -> Self {
        Self { protection }
    }

    pub fn protection(&self) -> i32 {
        self.protection
    }
}

impl Buildable for ItemArmorComponent {}

impl ItemComponent for ItemArmorComponent {
    fn serialize(&self) -> String {
        let value = self.protection;
        let val: String = ItemArmorComponentTemplate { protection: value }
            .render()
            .unwrap();
        val
    }
}

// * ItemCreativeCategoryComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/creative_category.json.jinja2",
    escape = "none"
)]
struct ItemCreativeCategoryComponentTemplate {
    parent: String,
}
#[derive(Clone)]
pub struct ItemCreativeCategoryComponent {
    parent: String,
}

impl Buildable for ItemCreativeCategoryComponent {}

impl ItemComponent for ItemCreativeCategoryComponent {
    fn serialize(&self) -> String {
        let parent = self.parent.clone();
        let val: String = ItemCreativeCategoryComponentTemplate { parent }
            .render()
            .unwrap();
        val
    }
}

impl ItemCreativeCategoryComponent {
    pub fn new(parent: impl Into<String>) -> Self {
        Self {
            parent: parent.into(),
        }
    }

    pub fn parent(&self) -> String {
        self.parent.clone()
    }
}

// * ItemRepairableComponent

#[derive(Template)]
#[template(
    path = "item_serialization/components/item_repair_entry.json.jinja2",
    escape = "none"
)]
struct ItemRepairEntryTemplate {
    items: String,
    amount: String,
}

#[derive(Clone)]
pub struct ItemRepairEntry {
    items: Vec<String>,
    amount: String,
}
impl ItemRepairEntry {
    pub fn serialize(&self) -> String {
        let items = format!("{:?}", self.items);
        let amount = self.amount.clone();
        let val: String = ItemRepairEntryTemplate { items, amount }.render().unwrap();
        val
    }

    pub fn new(items: Vec<impl Into<String> + Clone>, amount: impl Into<String>) -> Self {
        Self {
            items: items.iter().map(|x| (*x).clone().into()).collect(),
            amount: amount.into(),
        }
    }

    pub fn amount(&self) -> String {
        self.amount.clone()
    }

    pub fn items(&self) -> Vec<String> {
        self.items.clone()
    }
}

fn serialize_item_repairable_entries(repair_entries: &Vec<ItemRepairEntry>) -> String {
    let mut serialized_entries = String::new();
    for entry in repair_entries {
        serialized_entries.push_str(&entry.serialize());
        serialized_entries.push_str(",");
    }
    serialized_entries.pop();
    serialized_entries
}

#[derive(Template)]
#[template(
    path = "item_serialization/components/repairable.json.jinja2",
    escape = "none"
)]
struct ItemRepairableComponentTemplate {
    repair_entries: String,
}
#[derive(Clone)]
pub struct ItemRepairableComponent {
    repair_entries: Vec<ItemRepairEntry>,
}

impl Buildable for ItemRepairableComponent {}

impl ItemComponent for ItemRepairableComponent {
    fn serialize(&self) -> String {
        let repair_entries = &self.repair_entries;
        let val: String = ItemRepairableComponentTemplate {
            repair_entries: serialize_item_repairable_entries(repair_entries),
        }
        .render()
        .unwrap();
        val
    }
}

impl ItemRepairableComponent {
    pub fn new(repair_entries: Vec<ItemRepairEntry>) -> Self {
        Self { repair_entries }
    }

    pub fn repair_entries(&self) -> Vec<ItemRepairEntry> {
        self.repair_entries.clone()
    }
}

// ItemCustomComponents

#[derive(Template)]
#[template(
    path = "item_serialization/components/custom_components.json.jinja2",
    escape = "none"
)]
pub struct ItemCustomComponentsTemplate {
    pub components: String,
}

#[derive(Clone)]
pub struct ItemCustomComponents {
    components: Vec<Identifier>,
}

impl Buildable for ItemCustomComponents {}

impl ItemComponent for ItemCustomComponents {
    fn serialize(&self) -> String {
        let components_ser: Vec<String> = self.components.iter().map(|x| x.render()).collect();
        let components = format!("{:?}", components_ser);
        let val: String = ItemCustomComponentsTemplate { components }
            .render()
            .unwrap();
        val
    }
}

impl ItemCustomComponents {
    pub fn new(components: Vec<Identifier>) -> Self {
        Self { components }
    }

    pub fn components(&self) -> Vec<Identifier> {
        self.components.clone()
    }
}
