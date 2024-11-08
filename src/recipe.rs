use crate::vio::{Buildable, Identifier};
use askama::Template;

pub trait Recipe {
    fn serialize(&self) -> String;
    fn id(&self) -> Identifier;
}

#[derive(Clone)]
pub struct RecipeIO {
    pub use_tag: bool,
    pub item: Option<Identifier>,
    pub data: Option<i32>,
    pub count: Option<i32>,
    pub tag: Option<String>,
    pub key: Option<char>,
}

impl RecipeIO {
    pub fn new_typed(item: Identifier) -> Self {
        Self {
            use_tag: false,
            item: Some(item),
            data: Some(0),
            count: None,
            tag: None,
            key: None,
        }
    }

    pub fn new_tagged(tag: impl Into<String>) -> Self {
        Self {
            use_tag: true,
            item: None,
            data: None,
            count: None,
            tag: Some(tag.into()),
            key: None,
        }
    }

    pub fn using_key(&mut self, key: char) -> Self {
        let mut self_cloned: Self = self.clone();

        self_cloned.key = Some(key);

        self_cloned
    }

    pub fn using_count(&mut self, count: i32) -> Self {
        let mut self_cloned: Self = self.clone();

        self_cloned.count = Some(count);

        self_cloned
    }

    pub fn using_data(&mut self, data: i32) -> Self {
        let mut self_cloned: Self = self.clone();

        self_cloned.data = Some(data);

        self_cloned
    }
}

fn serialize_ingredients(ingredients: &Vec<RecipeIO>) -> String {
    let mut final_string = String::from("");
    for ingredient in ingredients {
        let val = RecipeInputTemplate {
            use_tag: ingredient.use_tag,
            item: ingredient
                .item
                .clone()
                .unwrap_or(Identifier::new("null", "null"))
                .render(),
            data: ingredient.data.unwrap_or(0),
            count: ingredient.count.unwrap_or(0),
            tag: ingredient
                .tag
                .clone()
                .unwrap_or("no".to_string())
                .to_string(),
            has_count: ingredient.count.is_some(),
            key: ingredient.key.unwrap_or('N').to_string(),
            has_key: ingredient.key.is_some(),
        }
        .render()
        .unwrap();
        final_string.push_str(val.as_str());
        final_string.push(',');
    }
    final_string.pop();
    final_string
}

#[derive(Template)]
#[template(
    path = "recipe_serialization/recipe_input.json.jinja2",
    escape = "none"
)]
struct RecipeInputTemplate {
    pub use_tag: bool,
    pub item: String,
    pub data: i32,
    pub count: i32,
    pub tag: String,
    pub has_count: bool,
    pub key: String,
    pub has_key: bool,
}

impl RecipeIO {
    pub fn serialize(&self) -> String {
        RecipeInputTemplate {
            use_tag: self.use_tag,
            item: self
                .item
                .clone()
                .unwrap_or(Identifier::new("null", "null"))
                .render(),
            data: self.data.unwrap_or(0),
            count: self.count.unwrap_or(0),
            tag: self.tag.clone().unwrap_or("null".to_string()).to_string(),
            has_count: self.count.is_some(),
            key: self.key.unwrap_or('N').to_string(),
            has_key: self.key.is_some(),
        }
        .render()
        .unwrap()
    }
}

// * FurnaceRecipe

#[derive(Clone)]
pub struct FurnaceRecipe {
    pub id: Identifier,
    pub tags: Vec<String>,
    pub input: RecipeIO,
    pub output: Identifier,
}
impl Recipe for FurnaceRecipe {
    fn serialize(&self) -> String {
        FurnaceRecipeTemplate {
            id: self.id.render(),
            tags: format!("{:?}", self.tags),
            input: self.input.serialize(),
            output: self.output.render(),
        }
        .render()
        .unwrap()
    }
    fn id(&self) -> Identifier {
        self.id.clone()
    }
}

impl FurnaceRecipe {
    pub fn new(id: Identifier, input: RecipeIO, output: Identifier) -> Self {
        Self {
            id,
            tags: vec![],
            input,
            output,
        }
    }

    pub fn using_tags(&mut self, tags: Vec<impl Into<String> + Clone>) -> Self {
        let mut clone_self = self.clone();

        clone_self.tags = tags.iter().map(|x| (*x).clone().into()).collect();

        clone_self
    }
}

impl Buildable for FurnaceRecipe {}

#[derive(Template)]
#[template(
    path = "recipe_serialization/furnace_recipe.json.jinja2",
    escape = "none"
)]
struct FurnaceRecipeTemplate {
    id: String,
    tags: String,
    input: String,
    output: String,
}

// * ShapelessRecipe

#[derive(Clone)]
pub struct ShapelessRecipe {
    pub id: Identifier,
    pub tags: Vec<String>,
    pub ingredients: Vec<RecipeIO>,
    pub result: RecipeIO,
}
impl Recipe for ShapelessRecipe {
    fn serialize(&self) -> String {
        let ingredients: &Vec<RecipeIO> = self.ingredients.as_ref();
        ShapelessRecipeTemplate {
            id: self.id.render(),
            tags: format!("{:?}", self.tags),
            ingredients: serialize_ingredients(ingredients),
            result: self.result.serialize(),
        }
        .render()
        .unwrap()
    }
    fn id(&self) -> Identifier {
        self.id.clone()
    }
}

impl Buildable for ShapelessRecipe {}

impl ShapelessRecipe {
    pub fn new(id: Identifier, result: RecipeIO) -> Self {
        Self {
            id,
            tags: vec![],
            ingredients: vec![],
            result,
        }
    }

    pub fn using_tags(&mut self, tags: Vec<impl Into<String> + Clone>) -> Self {
        let mut cloned_self = self.clone();

        cloned_self.tags = tags.iter().map(|x| (*x).clone().into()).collect();

        cloned_self
    }

    pub fn using_ingredients(&mut self, ingredients: Vec<RecipeIO>) -> Self {
        let mut cloned_self = self.clone();

        cloned_self.ingredients = ingredients;

        cloned_self
    }
}

#[derive(Template)]
#[template(
    path = "recipe_serialization/shapeless_recipe.json.jinja2",
    escape = "none"
)]
struct ShapelessRecipeTemplate {
    id: String,
    tags: String,
    ingredients: String,
    result: String,
}

// * ShapedRecipe

#[derive(Clone)]
pub struct ShapedRecipe {
    pub id: Identifier,
    pub tags: Vec<String>,
    pub ingredients: Vec<RecipeIO>,
    pub result: RecipeIO,
    pub pattern: Vec<String>,
}

impl Recipe for ShapedRecipe {
    fn serialize(&self) -> String {
        let ingredients: &Vec<RecipeIO> = self.ingredients.as_ref();
        ShapedRecipeTemplate {
            id: self.id.render(),
            tags: format!("{:?}", self.tags),
            ingredients: serialize_ingredients(ingredients),
            result: self.result.serialize(),
            pattern: format!("{:?}", self.pattern),
        }
        .render()
        .unwrap()
    }
    fn id(&self) -> Identifier {
        self.id.clone()
    }
}

impl ShapedRecipe {
    pub fn new(id: Identifier, result: RecipeIO) -> Self {
        Self {
            id,
            tags: vec![],
            ingredients: vec![],
            result,
            pattern: vec![],
        }
    }

    pub fn using_tags(&mut self, tags: Vec<impl Into<String> + Clone>) -> Self {
        let mut self_cloned: Self = self.clone();

        self_cloned.tags = tags.iter().map(|x| (*x).clone().into()).collect();

        self_cloned
    }

    pub fn using_pattern(&mut self, pattern: Vec<impl Into<String> + Clone>) -> Self {
        let mut self_cloned: Self = self.clone();

        self_cloned.pattern = pattern.iter().map(|x| (*x).clone().into()).collect();

        self_cloned
    }

    pub fn using_ingredients(&mut self, ingredients: Vec<RecipeIO>) -> Self {
        let mut self_cloned: Self = self.clone();

        self_cloned.ingredients = ingredients;

        self_cloned
    }
}

impl Buildable for ShapedRecipe {}

#[derive(Template)]
#[template(
    path = "recipe_serialization/shaped_recipe.json.jinja2",
    escape = "none"
)]
struct ShapedRecipeTemplate {
    id: String,
    tags: String,
    ingredients: String,
    result: String,
    pattern: String,
}
