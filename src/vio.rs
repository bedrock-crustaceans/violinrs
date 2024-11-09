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
        format!(
            "{}, {}, {}",
            self.major,
            self.minor,
            self.patch,
        )
    }
}
