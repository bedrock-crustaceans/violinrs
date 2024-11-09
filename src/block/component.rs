use askama::Template;

use crate::vio::{Buildable, Identifier, Pair, Vec3};

pub trait BlockComponent {
    fn serialize(&self) -> String;
}

// * BlockCollisionBoxComponent

#[derive(Clone)]
pub struct BlockCollisionBoxComponent {
    pub enabled: bool,
    pub origin: Option<Vec3>,
    pub size: Option<Vec3>,
}

impl Buildable for BlockCollisionBoxComponent {}

impl BlockCollisionBoxComponent {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            origin: None,
            size: None
        }
    }

    pub fn with_origin(&mut self, origin: Vec3) -> Self {
        let mut c = self.clone();

        c.origin = Some(origin);

        c
    }

    pub fn with_size(&mut self, size: Vec3) -> Self {
        let mut c = self.clone();

        c.size = Some(size);

        c
    }
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

#[derive(Clone)]
pub struct BlockCraftingTableComponent {
    pub name: String,
    pub tags: Vec<String>,
}

impl Buildable for BlockCraftingTableComponent {}

impl BlockCraftingTableComponentTemplate {
    pub fn new(name: impl Into<String>, tags: Vec<impl Into<String>>) -> Self {
        Self {
            name: name.into(),
            tags: tags.into_iter().map(|t| t.into()).collect(),
        }
    }
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

impl BlockComponent for BlockCraftingTableComponent {
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

#[derive(Clone)]
pub struct BlockDestructibleByExplosionComponent {
    pub explosion_resistance: Option<f64>,
}

impl Buildable for BlockDestructibleByExplosionComponent {}

impl BlockDestructibleByExplosionComponent {
    pub fn new(explosion_resistance: f64) -> Self {
        Self {
            explosion_resistance: Some(explosion_resistance)
        }
    }

    pub fn not_resistant() -> Self {
        Self {
            explosion_resistance: None
        }
    }
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

#[derive(Clone)]
pub struct BlockDestructibleByMiningComponent {
    pub seconds_to_destroy: Option<f64>,
}

impl Buildable for BlockDestructibleByMiningComponent {}

impl BlockDestructibleByMiningComponent {
    pub fn new(seconds_to_destroy: f64) -> Self {
        Self {
            seconds_to_destroy: Some(seconds_to_destroy)
        }
    }

    pub fn instant_mine() -> Self {
        Self {
            seconds_to_destroy: None
        }
    }
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

#[derive(Clone)]
pub struct BlockCustomComponents {
    pub components: Vec<Identifier>,
}

impl Buildable for BlockCustomComponents {}

impl BlockCustomComponents {
    pub fn new(components: Vec<Identifier>) -> Self {
        Self {
            components
        }
    }
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/custom_components.json.jinja2",
    escape = "none"
)]
struct BlockCustomComponentsTemplate {
    pub components: String,
}

impl BlockComponent for BlockCustomComponents {
    fn serialize(&self) -> String {
        let components_serialized: Vec<String> = self.components.iter().map(|x| x.render()).collect();
        BlockCustomComponentsTemplate {
            components: format!("{:?}", components_serialized),
        }
        .render()
        .unwrap()
    }
}

// * BlockDisplayNameComponent

#[derive(Clone)]
pub struct BlockDisplayNameComponent {
    pub value: String,
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/display_name.json.jinja2",
    escape = "none"
)]
struct BlockDisplayNameComponentTemplate {
    pub value: String,
}

impl Buildable for BlockDisplayNameComponent {}

impl BlockDisplayNameComponent {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl BlockComponent for BlockDisplayNameComponent {
    fn serialize(&self) -> String {
        BlockDisplayNameComponentTemplate {
            value: self.value.clone(),
        }
        .render()
        .unwrap()
    }
}

// * BlockFlammableComponent

#[derive(Clone)]
pub struct BlockFlammableComponent {
    pub catch_chance_modifier: i32,
    pub destroy_chance_modifier: i32,
}

impl Buildable for BlockFlammableComponent {}

impl BlockFlammableComponent {
    pub fn default() -> Self {
        Self {
            catch_chance_modifier: 5,
            destroy_chance_modifier: 20,
        }
    }

    pub fn new(catch_chance_modifier: i32, destroy_chance_modifier: i32) -> Self {
        Self {
            catch_chance_modifier,
            destroy_chance_modifier
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

#[derive(Clone)]
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

impl Buildable for BlockFrictionComponent {}

impl BlockFrictionComponent {
    pub fn new(friction: f64) -> Self {
        Self {
            friction
        }
    }
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

#[derive(Clone)]
pub struct BlockGeometryComponent {
    pub id: String,
    pub bone_visibility: Vec<Pair<String, bool>>,
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

impl Buildable for BlockGeometryComponent {}

impl BlockGeometryComponent {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            bone_visibility: vec![],
        }
    }

    pub fn using_bone_visibility(&mut self, bone_visibility: Vec<Pair<String, bool>>) -> Self {
        let mut c = self.clone();

        c.bone_visibility = bone_visibility;

        c
    }
}

impl BlockComponent for BlockGeometryComponent {
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

#[derive(Clone)]
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

impl Buildable for BlockLightDampeningComponent {}

impl BlockLightDampeningComponent {
    pub fn new(value: i32) -> Self {
        Self {
            value
        }
    }
}

impl BlockComponent for BlockLightDampeningComponent {
    fn serialize(&self) -> String {
        BlockLightDampeningComponentTemplate { value: self.value }
            .render()
            .unwrap()
    }
}

// * BlockLightDampeningComponent

#[derive(Clone)]
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

impl Buildable for BlockLightEmissionComponent {}

impl BlockLightEmissionComponent {
    pub fn new(value: i32) -> Self {
        Self {
            value
        }
    }
}

impl BlockComponent for BlockLightEmissionComponent {
    fn serialize(&self) -> String {
        BlockLightEmissionComponentTemplate { value: self.value }
            .render()
            .unwrap()
    }
}

// * BlockLootComponent

#[derive(Clone)]
pub struct BlockLootComponent {
    pub path: String,
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/loot.json.jinja2",
    escape = "none"
)]
struct BlockLootComponentTemplate {
    pub path: String,
}

impl Buildable for BlockLootComponent {}

impl BlockLootComponent {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
        }
    }
}

impl BlockComponent for BlockLootComponent {
    fn serialize(&self) -> String {
        BlockLootComponentTemplate {
            path: self.path.clone(),
        }
        .render()
        .unwrap()
    }
}

// * BlockMapColorComponent

#[derive(Clone)]
pub struct BlockMapColorComponent {
    pub color: String,
}

impl Buildable for BlockMapColorComponent {}

impl BlockMapColorComponent {
    pub fn new(color: impl Into<String>) -> Self {
        Self {
            color: color.into(),
        }
    }
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/map_color.json.jinja2",
    escape = "none"
)]
struct BlockMapColorComponentTemplate {
    pub color: String,
}

impl BlockComponent for BlockMapColorComponent {
    fn serialize(&self) -> String {
        BlockMapColorComponentTemplate {
            color: self.color.clone(),
        }
        .render()
        .unwrap()
    }
}

// TODO: IMPORTANT: TRANSFORM, SELECTION_BOX, MATERIAL_INSTANCES, PLACEMENT_FILTERS COMPONENTS
