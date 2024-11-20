use crate::vio::{Buildable, Identifier};
use askama::Template;

pub trait BlockState {
    fn serialize(&self) -> String;
}

#[derive(Clone)]
pub struct BoolBlockState {
    pub id: Identifier,
}

impl Buildable for BoolBlockState {}

#[derive(Template)]
#[template(
    path = "block_serialization/state/bool_state.json.jinja2",
    escape = "none"
)]
struct BoolBlockStateTemplate {
    identifier: String,
}

impl BlockState for BoolBlockState {
    fn serialize(&self) -> String {
        BoolBlockStateTemplate {
            identifier: self.id.render().to_string(),
        }
        .render()
        .unwrap()
    }
}

#[derive(Clone)]
pub struct NumericBlockState {
    pub id: Identifier,
    pub values: Vec<i32>,
}

impl Buildable for NumericBlockState {}

impl NumericBlockState {
    pub fn new(id: Identifier, values: Vec<i32>) -> Self {
        Self {
            id, values
        }
    }
}

#[derive(Template)]
#[template(
    path = "block_serialization/state/numeric_state.json.jinja2",
    escape = "none"
)]
struct VariableBlockStateTemplate {
    identifier: String,
    values: String,
}

impl BlockState for NumericBlockState {
    fn serialize(&self) -> String {
        VariableBlockStateTemplate {
            identifier: self.id.render().to_string(),
            values: format!("{:?}", self.values),
        }
        .render()
        .unwrap()
    }
}

#[derive(Clone)]
pub struct StringBlockState {
    pub id: Identifier,
    pub values: Vec<String>,
}

impl Buildable for StringBlockState {}

impl StringBlockState {
    pub fn new(id: Identifier, values: Vec<impl Into<String>>) -> Self {
        Self {
            id,
            values: values.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl BlockState for StringBlockState {
    fn serialize(&self) -> String {
        VariableBlockStateTemplate {
            identifier: self.id.render().to_string(),
            values: format!("{:?}", self.values),
        }
        .render()
        .unwrap()
    }
}

#[derive(Clone)]
pub struct RangedBlockState {
    pub id: Identifier,
    pub min: i32,
    pub max: i32,
}

impl Buildable for RangedBlockState {}

impl RangedBlockState {
    pub fn new(id: Identifier, min: i32, max: i32) -> Self {
        Self {
            id,
            min,
            max
        }
    }
}

#[derive(Template)]
#[template(
    path = "block_serialization/state/ranged_state.json.jinja2",
    escape = "none"
)]
struct RangedBlockStateTemplate {
    identifier: String,
    min: i32,
    max: i32,
}

impl BlockState for RangedBlockState {
    fn serialize(&self) -> String {
        RangedBlockStateTemplate {
            identifier: self.id.render().to_string(),
            min: self.min,
            max: self.max,
        }
        .render()
        .unwrap()
    }
}
