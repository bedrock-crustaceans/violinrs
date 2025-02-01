use crate::block::utils::{BlockFace, BlockPlacementCondition, MaterialInstance};
use crate::vio::{Buildable, Identifier, MolangStatement, RGBColor, Vec3};
use block_component_macros::block_component;
use std::collections::HashMap;

pub trait BlockComponent {
    fn serialize(&self) -> String;
}

// * BlockCollisionBoxComponent

// #[derive(Clone)]
// pub struct BlockCollisionBoxComponent {
//     pub enabled: bool,
//     pub origin: Option<Vec3>,
//     pub size: Option<Vec3>,
// }

block_component! {
    name = CollisionBox for "minecraft:collision_box";
    origin has Vec3 for "origin" with "public";
    size has Vec3 for "size" with "public";
}

impl BlockCollisionBoxComponent {
    pub fn full() -> Self {
        Self {
            origin: Vec3 {
                x: -8.0,
                y: 0.0,
                z: -8.0,
            },
            size: Vec3 {
                x: 16.0,
                y: 16.0,
                z: 16.0,
            },
        }
    }

    pub fn disabled() -> BlockNoCollisionBoxComponent {
        BlockNoCollisionBoxComponent::new(false)
    }
}

block_component! {
    name = NoCollisionBox for "minecraft:collision_box" with "transparency";
    enabled has bool for "minecraft:collision_box";
}

// * BlockCraftingTableBoxComponent

block_component! {
    name = CraftingTable for "minecraft:crafting_table";
    table_name has String for "table_name" with "public" "into";
    crafting_tags has Vec<String> for "crafting_tags" with "public";
}

// * BlockDestructibleByExplosionComponent

block_component! {
    name = DestructibleByExplosion for "minecraft:destructible_by_explosion";
    explosion_resistance has f64 for "explosion_resistance" with "public";
}

block_component! {
    name = IndestructibleByExplosion for "minecraft:destructible_by_explosion" with "transparency";
    destructible has bool for "destructible" with "public";
}

impl BlockDestructibleByExplosionComponent {
    pub fn indestructible() -> BlockIndestructibleByExplosionComponent {
        BlockIndestructibleByExplosionComponent::new(false)
    }
}

// * BlockDestructibleByMiningComponent

block_component! {
    name = DestructibleByMining for "minecraft:destructible_by_mining";
    seconds_to_destroy has f64 for "seconds_to_destroy" with "public";
}

block_component! {
    name = IndestructibleByMining for "minecraft:destructible_by_mining" with "transparency";
    destructible has bool for "destructible" with "public";
}

impl BlockDestructibleByMiningComponent {
    pub fn instant_mine() -> Self {
        Self {
            seconds_to_destroy: 0.0,
        }
    }

    pub fn indestructible() -> BlockIndestructibleByMiningComponent {
        BlockIndestructibleByMiningComponent::new(false)
    }
}

// * BlockCustomComponents

block_component! {
    name = CustomComponents for "minecraft:custom_components" with "transparency";
    components has Vec<Identifier> for "components" with "public";
}

// * BlockDisplayNameComponent

block_component! {
    name = DisplayName for "minecraft:display_name";
    value has String for "value" with "public" "into";
}

// * BlockFlammableComponent

block_component! {
    name = Flammable for "minecraft:flammable";
    catch_chance_modifier has i32 for "catch_chance_modifier" with "public";
    destroy_chance_modifier has i32 for "destroy_chance_modifier" with "public";
}

impl BlockFlammableComponent {
    pub fn default() -> Self {
        Self {
            catch_chance_modifier: 5,
            destroy_chance_modifier: 2,
        }
    }
}

// * BlockFrictionComponent

block_component! {
    name = Friction for "minecraft:friction" with "transparency";
    friction has f64 for "friction" with "public";
}

// * BlockGeometryComponent

block_component! {
    name = Geometry for "minecraft:geometry";
    identifier has String for "identifier" with "public" "into";
    bone_visibility has HashMap<String, MolangStatement> for "bone_visibility" with "public";
}

// * BlockLightDampeningComponent

block_component! {
    name = LightDampening for "minecraft:light_dampening" with "transparency";
    value has u8 for "value" with "public";
}

// * BlockLightEmissionComponent

block_component! {
    name = LightEmission for "minecraft:light_emission" with "transparency";
    value has u8 for "value" with "public";
}

// * BlockLootComponent

block_component! {
    name = Loot for "minecraft:loot" with "transparency";
    path has String for "path" with "public" "into";
}

// * BlockMapColorComponent

block_component! {
    name = MapColor for "minecraft:map_color" with "transparency";
    color has RGBColor for "color" with "public";
}

// * MaterialInstances

block_component! {
    name = MaterialInstances for "minecraft:material_instances" with "transparency";
    instances has HashMap<BlockFace, MaterialInstance> for "instances" with "public";
}

// * RedstoneConductivity

block_component! {
    name = RedstoneConductivity for "minecraft:redstone_conductivity";
    redstone_conductor has bool for "redstone_conductor" with "public";
    allows_wire_to_step_down has bool for "allows_wire_to_step_down" with "public";
}

// * SelectionBox

block_component! {
    name = SelectionBox for "minecraft:selection_box";
    origin has Vec3 for "origin" with "public";
    size has Vec3 for "size" with "public";
}

impl BlockSelectionBoxComponent {
    pub fn full() -> Self {
        Self {
            origin: Vec3 {
                x: -8.0,
                y: 0.0,
                z: -8.0,
            },
            size: Vec3 {
                x: 16.0,
                y: 16.0,
                z: 16.0,
            },
        }
    }

    pub fn disabled() -> BlockNoSelectionBoxComponent {
        BlockNoSelectionBoxComponent::new(false)
    }
}

block_component! {
    name = NoSelectionBox for "minecraft:selection_box" with "transparency";
    enabled has bool for "minecraft:selection_box";
}

// * Tick

block_component! {
    name = Tick for "minecraft:tick";
    interval_range has [i32; 2] for "interval_range" with "public";
    looping has bool for "looping" with "public";
}

// * Transformation

block_component! {
    name = Transformation for "minecraft:transformation";
    translation has Vec3 for "translation" with "public";
    rotation has Vec3 for "rotation" with "public";
    scale has Vec3 for "scale" with "public";
    rotation_pivot has Vec3 for "rotation_pivot" with "public";
    scale_pivot has Vec3 for "scale_pivot" with "public";
}

// * PlacementFilter

block_component! {
    name = PlacementFilter for "minecraft:placement_filter";
    conditions has Vec<BlockPlacementCondition> for "conditions" with "public";
}

// * EntityFallOn

block_component! {
    name = EntityFallOn for "minecraft:entity_fall_on";
    min_fall_distance has f64 for "min_fall_distance" with "public";
}
