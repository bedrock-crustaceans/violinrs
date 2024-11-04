use std::sync::Arc;
use askama::Template;
use crate::vio::Buildable;

pub trait ItemComponent : Buildable {
    fn serialize(&self) -> String;
}

// * ItemDamageComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/damage.json.jinja2",
    escape = "none"
)]
struct ItemDamageComponentTemplate {
    value: i32,
}
pub struct ItemDamageComponent {
    pub value: i32,
}

impl Buildable for ItemDamageComponent {}

impl ItemComponent for ItemDamageComponent {
    fn serialize(&self) -> String {
        let value = self.value;
        let val = ItemDamageComponentTemplate { value }
            .render()
            .unwrap();
        val
    }
}

// * ItemDisplayNameComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/display_name.json.jinja2",
    escape = "none"
)]
struct ItemDisplayNameComponentTemplate<'a> {
    value: &'a str,
}
pub struct ItemDisplayNameComponent<'a> {
    pub value: &'a str,
}

impl<'a> Buildable for ItemDisplayNameComponent<'a> {}

impl<'a> ItemComponent for ItemDisplayNameComponent<'a> {
    fn serialize(&self) -> String {
        let value = self.value;
        let val: String = ItemDisplayNameComponentTemplate { value }
            .render()
            .unwrap();
        val
    }
}

// * ItemIconComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/icon.json.jinja2",
    escape = "none"
)]
struct ItemIconComponentTemplate<'a> {
    texture: &'a str,
}
pub struct ItemIconComponent<'a> {
    pub texture: &'a str,
}

impl<'a> Buildable for ItemIconComponent<'a> {}

impl<'a> ItemComponent for ItemIconComponent<'a> {
    fn serialize(&self) -> String {
        let value = self.texture;
        let val: String = ItemIconComponentTemplate { texture: value }
            .render()
            .unwrap();
        val
    }
}

// * ItemFuelComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/fuel.json.jinja2",
    escape = "none"
)]
struct ItemFuelComponentTemplate {
    duration: i32,
}
pub struct ItemFuelComponent {
    pub duration: i32,
}

impl Buildable for ItemFuelComponent {}

impl ItemComponent for ItemFuelComponent {
    fn serialize(&self) -> String {
        let value = self.duration;
        let val: String = ItemFuelComponentTemplate { duration: value }
            .render()
            .unwrap();
        val
    }
}

// * ItemHandEquippedComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/hand_equipped.json.jinja2",
    escape = "none"
)]
struct ItemHandEquippedComponentTemplate {
    value: bool,
}
pub struct ItemHandEquippedComponent {
    pub value: bool,
}

impl Buildable for ItemHandEquippedComponent {}

impl ItemComponent for ItemHandEquippedComponent {
    fn serialize(&self) -> String {
        let value = self.value;
        let val: String = ItemHandEquippedComponentTemplate { value }
            .render()
            .unwrap();
        val
    }
}

// * ItemAllowOffHandComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/allow_off_hand.json.jinja2",
    escape = "none"
)]
struct ItemAllowOffHandComponentTemplate {
    value: bool,
}
pub struct ItemAllowOffHandComponent {
    pub value: bool,
}

impl Buildable for ItemAllowOffHandComponent {}

impl ItemComponent for ItemAllowOffHandComponent {
    fn serialize(&self) -> String {
        let value = self.value;
        let val: String = ItemAllowOffHandComponentTemplate { value }
            .render()
            .unwrap();
        val
    }
}

// * ItemMaxStackValueComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/max_stack_value.json.jinja2",
    escape = "none"
)]
struct ItemMaxStackValueComponentTemplate {
    value: bool,
}
pub struct ItemMaxStackValueComponent {
    pub value: bool,
}

impl Buildable for ItemMaxStackValueComponent {}

impl ItemComponent for ItemMaxStackValueComponent {
    fn serialize(&self) -> String {
        let value = self.value;
        let val: String = ItemMaxStackValueComponentTemplate { value }
            .render()
            .unwrap();
        val
    }
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

// * ItemArmorComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/armor.json.jinja2",
    escape = "none"
)]
struct ItemArmorComponentTemplate {
    protection: i32,
}
pub struct ItemArmorComponent {
    pub protection: i32,
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

// * ItemRenderOffsetsComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/render_offsets.json.jinja2",
    escape = "none"
)]
struct ItemRenderOffsetsComponentTemplate<'a> {
    value: &'a str,
}
pub struct ItemRenderOffsetsComponent<'a> {
    pub value: &'a str,
}

impl<'a> Buildable for ItemRenderOffsetsComponent<'a> {}

impl<'a> ItemComponent for ItemRenderOffsetsComponent<'a> {
    fn serialize(&self) -> String {
        let value = self.value;
        let val: String = ItemRenderOffsetsComponentTemplate { value }
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
struct ItemCreativeCategoryComponentTemplate<'a> {
    parent: &'a str,
}
pub struct ItemCreativeCategoryComponent<'a> {
    pub parent: &'a str,
}

impl<'a> Buildable for ItemCreativeCategoryComponent<'a> {}

impl<'a> ItemComponent for ItemCreativeCategoryComponent<'a> {
    fn serialize(&self) -> String {
        let parent = self.parent;
        let val: String = ItemCreativeCategoryComponentTemplate { parent }
            .render()
            .unwrap();
        val
    }
}

// * ItemRepairableComponent

#[derive(Template)]
#[template(
    path = "item_serialization/components/item_repair_entry.json.jinja2",
    escape = "none"
)]
struct ItemRepairEntryTemplate<'a> {
    items: String,
    amount: &'a String,
}

pub struct ItemRepairEntry<'a> {
    pub items: Vec<&'a str>,
    pub amount: String,
}
impl<'a> ItemRepairEntry<'a> {
    pub fn serialize(&self) -> String {
        let items = format!("{:?}", self.items);
        let amount = &self.amount;
        let val: String = ItemRepairEntryTemplate { items, amount }.render().unwrap();
        val
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
pub struct ItemRepairableComponent<'a> {
    pub repair_entries: Vec<ItemRepairEntry<'a>>,
}

impl<'a> Buildable for ItemRepairableComponent<'a> {}

impl<'a> ItemComponent for ItemRepairableComponent<'a> {
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

// ItemCustomComponents

#[derive(Template)]
#[template(
    path = "item_serialization/components/custom_components.json.jinja2",
    escape = "none"
)]
pub struct ItemCustomComponentsTemplate {
    pub components: String,
}

pub struct ItemCustomComponents<'a> {
    pub components: Vec<&'a str>,
}

impl<'a> Buildable for ItemCustomComponents<'a> {}

impl<'a> ItemComponent for ItemCustomComponents<'a> {
    fn serialize(&self) -> String {
        let components = format!("{:?}", self.components);
        let val: String = ItemCustomComponentsTemplate { components }
            .render()
            .unwrap();
        val
    }
}
