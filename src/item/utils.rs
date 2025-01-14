use serde::{Deserialize, Serialize, Serializer};
use crate::vio::Identifier;

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

#[derive(Clone, Debug)]
#[derive(Serialize)]
pub struct DurabilityThreshold {
    durability: i32,
    sound_effect: Option<String>,
    particle_type: Option<Identifier>
}

impl DurabilityThreshold {
    pub fn new(durability: i32) -> Self {
        Self {
            durability,
            sound_effect: None,
            particle_type: None
        }
    }

    pub fn using_sound_effect(&mut self, src: impl Into<String>) -> Self {
        let mut sc = self.clone();

        sc.sound_effect = Some(src.into());

        sc
    }

    pub fn using_particle_type(&mut self, src: Identifier) -> Self {
        let mut sc = self.clone();

        sc.particle_type = Some(src);

        sc
    }
}

#[derive(Clone, Debug)]
#[derive(Serialize)]
pub struct ItemTextureDescriptor {
    default: String,
    dyed: Option<String>,
    icon_trim: Option<String>
}

impl ItemTextureDescriptor {
    pub fn new(default: impl Into<String>) -> Self {
        Self {
            default: default.into(),
            dyed: None,
            icon_trim: None

        }
    }

    pub fn using_dyed(&mut self, src: impl Into<String>) -> Self {
        let mut sc = self.clone();

        sc.dyed = Some(src.into());

        sc
    }

    pub fn using_icon_trim(&mut self, src: impl Into<String>) -> Self {
        let mut sc = self.clone();

        sc.icon_trim = Some(src.into());

        sc
    }
}

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

#[derive(Clone, Debug)]
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemRarity {
    Rare,
    Common,
    Uncommon,
    Epic
}

#[derive(Clone, Debug)]
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemAnimation {
    Eat,
    Drink,
    Bow,
    Block,
    Camera,
    Crossbow,
    None,
    Brush,
    Spear,
    Spyglass
}

#[derive(Clone, Debug)]
pub enum ItemWearableSlot {
    WeaponOffhand,
    ArmorHead,
    ArmorChest,
    ArmorLegs,
    ArmorFeet
}

impl ItemWearableSlot {
    pub fn str_slot(&self) -> &str {
        match self {
            ItemWearableSlot::WeaponOffhand => "slot.weapon.offhand",
            ItemWearableSlot::ArmorHead => "slot.armor.head",
            ItemWearableSlot::ArmorChest => "slot.armor.chest",
            ItemWearableSlot::ArmorLegs => "slot.armor.legs",
            ItemWearableSlot::ArmorFeet => "slot.armor.feet"
        }
    }
}

impl Serialize for ItemWearableSlot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.str_slot())
    }
}