use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy)]
pub struct Identifier<'a> {
    pub namespace: &'a str,
    pub value: &'a str,
}

impl Identifier<'_> {
    pub fn render(&self) -> String {
        format!("{}:{}", self.namespace, self.value)
    }
}

pub struct Pair<T, K> {
    pub first: T,
    pub second: K
}

pub trait Buildable {
    fn build(&self) -> Arc<Self> {
        Arc::new(self.clone())
    }
}
