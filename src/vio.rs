use derive_setters::Setters;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize, Serializer};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn render_as_arr(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

impl Serialize for Vec3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;

        for e in self.render_as_arr() {
            seq.serialize_element(&e)?
        }

        seq.end()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Identifier {
    pub namespace: String,
    pub value: String,
}

impl Identifier {
    pub fn render(&self) -> String {
        format!("{}:{}", self.namespace, self.value)
    }

    pub fn new(namespace: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            value: value.into(),
        }
    }
}

impl From<(String, String)> for Identifier {
    fn from(value: (String, String)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(&str, &str)> for Identifier {
    fn from(value: (&str, &str)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.render())
    }
}

#[derive(Clone)]
pub struct Pair<T: Clone, K: Clone> {
    pub first: T,
    pub second: K,
}

impl<T: Clone, K: Clone> Pair<T, K> {
    pub fn new(first: T, second: K) -> Self {
        Self { first, second }
    }
}

pub trait Buildable: Clone {
    fn build(&self) -> Arc<Self> {
        Arc::new(self.clone())
    }
}

pub trait Generatable {
    fn generate(&self, path_buf: impl Into<PathBuf>);
}

#[derive(Clone)]
pub struct SemVer {
    major: i32,
    minor: i32,
    patch: i32,
    beta: bool,
}

impl SemVer {
    pub fn new(major: i32, minor: i32, patch: i32) -> Self {
        Self {
            major,
            minor,
            patch,
            beta: false,
        }
    }

    pub fn new_beta(major: i32, minor: i32, patch: i32) -> Self {
        Self {
            major,
            minor,
            patch,
            beta: true,
        }
    }

    pub fn render(&self) -> String {
        format!(
            "{}.{}.{}{}",
            self.major,
            self.minor,
            self.patch,
            if self.beta { "-beta" } else { "" }
        )
    }

    pub fn render_commas(&self) -> String {
        format!("{}, {}, {}", self.major, self.minor, self.patch,)
    }

    pub fn current() -> Self {
        Self {
            major: 1,
            minor: 21,
            patch: 50,
            beta: false,
        }
    }
}

impl Default for SemVer {
    fn default() -> Self {
        Self::current()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RangeDescriptor<T>
where
    T: Clone,
{
    pub min: T,
    pub max: T,
}

impl<T> RangeDescriptor<T>
where
    T: Clone,
{
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

pub fn vec_into<T>(vec: Vec<impl Into<T>>) -> Vec<T>
where
    T: Clone,
{
    let new_vec = vec.into_iter().map(|e| e.into()).collect();

    new_vec
}

pub trait VecInto<T, E>
where
    T: Into<E>,
{
    fn vec_into(&self) -> Vec<E>
    where
        T: Clone;
}

impl<T, E> VecInto<T, E> for Vec<T>
where
    T: Into<E> + Clone,
    E: From<T>,
{
    fn vec_into(&self) -> Vec<E> {
        let new_vec = self.into_iter().map(|e| e.clone().into()).collect();

        new_vec
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(transparent)]
pub struct MolangStatement(String);

impl MolangStatement {
    pub fn new(src: impl Into<String>) -> Self {
        MolangStatement(src.into())
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ColorCode {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    #[default]
    White,
    MinecoinGold,
    MaterialQuartz,
    MaterialIron,
    MaterialNetherite,
    MaterialRedstone,
    MaterialCopper,
    MaterialGold,
    MaterialEmerald,
    MaterialDiamond,
    MaterialLapis,
    MaterialAmethyst,
    MaterialResin,
}

impl ColorCode {
    /// The color code for use in strings. (in form of `§X`)
    pub fn str_code(&self) -> &str {
        match self {
            ColorCode::Black => "§0",
            ColorCode::DarkBlue => "§1",
            ColorCode::DarkGreen => "§2",
            ColorCode::DarkAqua => "§3",
            ColorCode::DarkRed => "§4",
            ColorCode::DarkPurple => "§5",
            ColorCode::Gold => "§6",
            ColorCode::Gray => "§7",
            ColorCode::DarkGray => "§8",
            ColorCode::Blue => "§9",
            ColorCode::Green => "§a",
            ColorCode::Aqua => "§b",
            ColorCode::Red => "§c",
            ColorCode::LightPurple => "§d",
            ColorCode::Yellow => "§e",
            ColorCode::White => "§f",
            ColorCode::MinecoinGold => "§g",
            ColorCode::MaterialQuartz => "§h",
            ColorCode::MaterialIron => "§i",
            ColorCode::MaterialNetherite => "§j",
            ColorCode::MaterialRedstone => "§m",
            ColorCode::MaterialCopper => "§n",
            ColorCode::MaterialGold => "§p",
            ColorCode::MaterialEmerald => "§q",
            ColorCode::MaterialDiamond => "§s",
            ColorCode::MaterialLapis => "§t",
            ColorCode::MaterialAmethyst => "§u",
            ColorCode::MaterialResin => "§v",
        }
    }
}

#[derive(Clone, Copy, Debug, Setters)]
#[setters(prefix = "using_")]
pub struct RGBColor {
    red: u8,
    green: u8,
    blue: u8,
    preferred_serialization_way: ColorSerializationWay,
}

impl RGBColor {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            preferred_serialization_way: ColorSerializationWay::Arr,
        }
    }

    pub fn render_as_arr(&self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }

    pub fn render_as_hex(&self) -> String {
        format!(
            "#{}{}{}",
            Self::component_to_string(self.red),
            Self::component_to_string(self.green),
            Self::component_to_string(self.blue)
        )
    }

    fn component_to_string(v: u8) -> String {
        format!("{:X}", v)
    }
}

impl Serialize for RGBColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.preferred_serialization_way {
            ColorSerializationWay::Hex => serializer.serialize_str(&self.render_as_hex()),
            ColorSerializationWay::Arr => {
                let mut seq = serializer.serialize_seq(Some(self.render_as_arr().len()))?;

                for item in &self.render_as_arr() {
                    seq.serialize_element(item)?;
                }

                seq.end()
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ColorSerializationWay {
    Hex,
    Arr,
}

pub trait ViolaDefault {
    fn viola_default() -> Self;
}
