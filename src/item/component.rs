use crate::vio::{Buildable, Identifier};
use askama::Template;

pub trait ItemComponent {
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
#[derive(Clone)]
pub struct ItemDamageComponent {
    value: i32,
}

impl ItemDamageComponent {
    pub fn new(value: i32) -> Self {
        ItemDamageComponent { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

impl Buildable for ItemDamageComponent {}

impl ItemComponent for ItemDamageComponent {
    fn serialize(&self) -> String {
        let value = self.value;
        let val = ItemDamageComponentTemplate { value }.render().unwrap();
        val
    }
}

// * ItemDisplayNameComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/display_name.json.jinja2",
    escape = "none"
)]
struct ItemDisplayNameComponentTemplate {
    value: String,
}
#[derive(Clone)]
pub struct ItemDisplayNameComponent {
    value: String,
}

impl Buildable for ItemDisplayNameComponent {}

impl ItemComponent for ItemDisplayNameComponent {
    fn serialize(&self) -> String {
        let value = self.value.clone();
        let val: String = ItemDisplayNameComponentTemplate { value }.render().unwrap();
        val
    }
}

impl ItemDisplayNameComponent {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn name(&self) -> String {
        self.value.clone()
    }
}

// * ItemIconComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/icon.json.jinja2",
    escape = "none"
)]
struct ItemIconComponentTemplate {
    texture: String,
}
#[derive(Clone)]
pub struct ItemIconComponent {
    texture: String,
}

impl Buildable for ItemIconComponent {}

impl ItemComponent for ItemIconComponent {
    fn serialize(&self) -> String {
        let value = self.texture.clone();
        let val: String = ItemIconComponentTemplate { texture: value }
            .render()
            .unwrap();
        val
    }
}

impl ItemIconComponent {
    pub fn new(texture: impl Into<String>) -> Self {
        Self {
            texture: texture.into(),
        }
    }

    pub fn texture(&self) -> String {
        self.texture.clone()
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
#[derive(Clone)]
pub struct ItemFuelComponent {
    duration: i32,
}

impl ItemFuelComponent {
    pub fn new(duration: i32) -> Self {
        Self { duration }
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }
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
#[derive(Clone)]
pub struct ItemHandEquippedComponent {
    value: bool,
}

impl ItemHandEquippedComponent {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn value(&self) -> bool {
        self.value
    }
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
#[derive(Clone)]
pub struct ItemAllowOffHandComponent {
    value: bool,
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

impl ItemAllowOffHandComponent {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn value(&self) -> bool {
        self.value
    }
}

// * ItemMaxStackValueComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/max_stack_value.json.jinja2",
    escape = "none"
)]
struct ItemMaxStackValueComponentTemplate {
    value: i32,
}
#[derive(Clone)]
pub struct ItemMaxStackValueComponent {
    value: i32,
}

impl ItemMaxStackValueComponent {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
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
