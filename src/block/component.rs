use askama::Template;

use crate::vio::{Pair, Vec3};

pub trait BlockComponent {
    fn serialize(&self) -> String;
}

// * BlockCollisionBoxComponent

pub struct BlockCollisionBoxComponent {
    pub enabled: bool,
    pub origin: Option<Vec3>,
    pub size: Option<Vec3>,
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/collision_box.json.jinja2",
    escape = "none"
)]
struct BlockCollisionBoxComponentTemplate {
    pub enabled: bool,
    pub origin: String,
    pub size: String,
}

impl BlockCollisionBoxComponent {
    pub fn full() -> Self {
        Self {
            enabled: true,
            origin: Some(Vec3 {
                x: -8.0,
                y: 0.0,
                z: -8.0,
            }),
            size: Some(Vec3 {
                x: 16.0,
                y: 16.0,
                z: 16.0,
            }),
        }
    }
}

impl BlockComponent for BlockCollisionBoxComponent {
    fn serialize(&self) -> String {
        let enabled = self.enabled;
        let orgn = self.origin.unwrap_or(Vec3 {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        });
        let sz = self.size.unwrap_or(Vec3 {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        });
        let origin = if orgn.x != -1.0 {
            format!("[{}, {}, {}]", orgn.x, orgn.y, orgn.z)
        } else {
            "[]".to_string()
        };
        let size = if sz.x != -1.0 {
            format!("[{}, {}, {}]", sz.x, sz.y, sz.z)
        } else {
            "[]".to_string()
        };
        BlockCollisionBoxComponentTemplate {
            enabled,
            origin,
            size,
        }
        .render()
        .unwrap()
    }
}

// * BlockCraftingTableBoxComponent

pub struct BlockCraftingTableComponent<'a> {
    pub name: &'a str,
    pub tags: Vec<&'a str>,
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/crafting_table.json.jinja2",
    escape = "none"
)]
struct BlockCraftingTableComponentTemplate {
    name: String,
    tags: String,
}

impl<'a> BlockComponent for BlockCraftingTableComponent<'a> {
    fn serialize(&self) -> String {
        let tags = format!("{:?}", self.tags);
        BlockCraftingTableComponentTemplate {
            tags,
            name: self.name.to_string(),
        }
        .render()
        .unwrap()
    }
}

// * BlockDestructibleByExplosionComponent

pub struct BlockDestructibleByExplosionComponent {
    pub explosion_resistance: Option<f64>,
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/destructible_by_explosion.json.jinja2",
    escape = "none"
)]
struct BlockDestructibleByExplosionComponentTemplate {
    pub explosion_resistance: f64,
}

impl BlockComponent for BlockDestructibleByExplosionComponent {
    fn serialize(&self) -> String {
        BlockDestructibleByExplosionComponentTemplate {
            explosion_resistance: self.explosion_resistance.unwrap_or(0.0),
        }
        .render()
        .unwrap()
    }
}

// * BlockDestructibleByMiningComponent

pub struct BlockDestructibleByMiningComponent {
    pub seconds_to_destroy: Option<f64>,
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/destructible_by_mining.json.jinja2",
    escape = "none"
)]
struct BlockDestructibleByMiningComponentTemplate {
    pub seconds_to_destroy: f64,
}

impl BlockComponent for BlockDestructibleByMiningComponent {
    fn serialize(&self) -> String {
        BlockDestructibleByMiningComponentTemplate {
            seconds_to_destroy: self.seconds_to_destroy.unwrap_or(0.0),
        }
        .render()
        .unwrap()
    }
}

// * BlockCustomComponents

pub struct BlockCustomComponents<'a> {
    pub components: Vec<&'a str>,
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/custom_components.json.jinja2",
    escape = "none"
)]
struct BlockCustomComponentsTemplate {
    pub components: String,
}

impl BlockComponent for BlockCustomComponents<'_> {
    fn serialize(&self) -> String {
        BlockCustomComponentsTemplate {
            components: format!("{:?}", self.components),
        }
        .render()
        .unwrap()
    }
}

// * BlockDisplayNameComponent

pub struct BlockDisplayNameComponent<'a> {
    pub value: &'a str,
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/display_name.json.jinja2",
    escape = "none"
)]
struct BlockDisplayNameComponentTemplate {
    pub value: String,
}

impl BlockComponent for BlockDisplayNameComponent<'_> {
    fn serialize(&self) -> String {
        BlockDisplayNameComponentTemplate {
            value: self.value.to_string(),
        }
        .render()
        .unwrap()
    }
}

// * BlockFlammableComponent

pub struct BlockFlammableComponent {
    pub catch_chance_modifier: i32,
    pub destroy_chance_modifier: i32,
}

impl BlockFlammableComponent {
    pub fn default() -> Self {
        Self {
            catch_chance_modifier: 5,
            destroy_chance_modifier: 20,
        }
    }
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/flammable.json.jinja2",
    escape = "none"
)]
struct BlockFlammableComponentTemplate {
    pub catch_chance_modifier: i32,
    pub destroy_chance_modifier: i32,
}

impl BlockComponent for BlockFlammableComponent {
    fn serialize(&self) -> String {
        BlockFlammableComponentTemplate {
            catch_chance_modifier: self.catch_chance_modifier.clone(),
            destroy_chance_modifier: self.destroy_chance_modifier,
        }
        .render()
        .unwrap()
    }
}

// * BlockFrictionComponent

pub struct BlockFrictionComponent {
    pub friction: f64,
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/friction.json.jinja2",
    escape = "none"
)]
struct BlockFrictionComponentTemplate {
    pub friction: f64,
}

impl BlockComponent for BlockFrictionComponent {
    fn serialize(&self) -> String {
        BlockFrictionComponentTemplate {
            friction: self.friction,
        }
        .render()
        .unwrap()
    }
}

// * BlockGeometryComponent

pub struct BlockGeometryComponent<'a> {
    pub id: &'a str,
    pub bone_visibility: Vec<Pair<&'a str, bool>>,
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/geometry.json.jinja2",
    escape = "none"
)]
struct BlockGeometryComponentTemplate {
    id: String,
    bone_visibility: String,
}

impl BlockComponent for BlockGeometryComponent<'_> {
    fn serialize(&self) -> String {
        let mut bv = String::from("");

        for entry in self.bone_visibility.iter() {
            bv.push_str(format!("\"{}\": {}", entry.first, entry.second).as_str());
            bv.push(',');
        }
        bv.pop();

        BlockGeometryComponentTemplate {
            id: self.id.to_string(),
            bone_visibility: bv,
        }
        .render()
        .unwrap()
    }
}

// * BlockLightDampeningComponent

pub struct BlockLightDampeningComponent {
    pub value: i32,
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/light_dampening.json.jinja2",
    escape = "none"
)]
struct BlockLightDampeningComponentTemplate {
    pub value: i32,
}

impl BlockComponent for BlockLightDampeningComponent {
    fn serialize(&self) -> String {
        BlockLightDampeningComponentTemplate { value: self.value }
            .render()
            .unwrap()
    }
}

// * BlockLightDampeningComponent

pub struct BlockLightEmissionComponent {
    pub value: i32,
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/light_emission.json.jinja2",
    escape = "none"
)]
struct BlockLightEmissionComponentTemplate {
    pub value: i32,
}

impl BlockComponent for BlockLightEmissionComponent {
    fn serialize(&self) -> String {
        BlockLightEmissionComponentTemplate { value: self.value }
            .render()
            .unwrap()
    }
}

// * BlockLootComponent

pub struct BlockLootComponent<'a> {
    pub path: &'a str,
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/loot.json.jinja2",
    escape = "none"
)]
struct BlockLootComponentTemplate {
    pub path: String,
}

impl BlockComponent for BlockLootComponent<'_> {
    fn serialize(&self) -> String {
        BlockLootComponentTemplate {
            path: self.path.to_string(),
        }
        .render()
        .unwrap()
    }
}

// * BlockMapColorComponent

pub struct BlockMapColorComponent<'a> {
    pub color: &'a str,
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/map_color.json.jinja2",
    escape = "none"
)]
struct BlockMapColorComponentTemplate {
    pub color: String,
}

impl BlockComponent for BlockMapColorComponent<'_> {
    fn serialize(&self) -> String {
        BlockMapColorComponentTemplate {
            color: self.color.to_string(),
        }
        .render()
        .unwrap()
    }
}

// TODO: IMPORTANT: TRANSFORM, SELECTION_BOX, MATERIAL_INSTANCES, PLACEMENT_FILTERS COMPONENTS
