use serde::Serialize;

#[derive(Clone, Debug)]
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EnchantableSlot {
    ArmorFeet,
    ArmorTorso,
    ArmorHead,
    ArmorLegs,
    Axe,
    Bow,
    CosmeticHead,
    Crossbow,
    Elytra,
    FishingRod,
    FlintSteel,
    Hoe,
    Pickaxe,
    Shears,
    Shield,
    Shovel,
    Sword,
    All
}