use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone)]
pub struct Identifier {
    namespace: String,
    value: String,
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

pub struct Pair<T, K> {
    pub first: T,
    pub second: K,
}

pub trait Buildable : Clone {
    fn build(&self) -> Arc<Self> {
        Arc::new(self.clone())
    }
}

#[derive(Clone)]
pub struct SemVer {
    major: i32,
    minor: i32,
    patch: i32
}

impl SemVer {
    pub fn new(major: i32, minor: i32, patch: i32) -> Self {
        Self {
            major,
            minor,
            patch
        }
    }

    pub fn render(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}