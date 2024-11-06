use askama::Template;

pub trait Recipe {
    fn serialize(&self) -> String;
    fn id(&self) -> String;
}

pub struct RecipeInputOrOutput<'a> {
    pub use_tag: bool,
    pub item: Option<&'a str>,
    pub data: Option<i32>,
    pub count: Option<i32>,
    pub tag: Option<&'a str>,
    pub key: Option<&'a str>,
}

fn serialize_ingredients(ingredients: &Vec<RecipeInputOrOutput>) -> String {
    let mut final_string = String::from("");
    for ingredient in ingredients {
        let val = RecipeInputTemplate {
            use_tag: ingredient.use_tag,
            item: ingredient.item.unwrap_or("null").to_string(),
            data: ingredient.data.unwrap_or(0),
            count: ingredient.count.unwrap_or(0),
            tag: ingredient.tag.unwrap_or("null").to_string(),
            has_count: ingredient.count.is_some(),
            key: ingredient.key.unwrap_or("null").to_string(),
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

impl<'a> RecipeInputOrOutput<'a> {
    pub fn serialize(&self) -> String {
        RecipeInputTemplate {
            use_tag: self.use_tag,
            item: self.item.unwrap_or("null").to_string(),
            data: self.data.unwrap_or(0),
            count: self.count.unwrap_or(0),
            tag: self.tag.unwrap_or("null").to_string(),
            has_count: self.count.is_some(),
            key: self.key.unwrap_or("null").to_string(),
            has_key: self.key.is_some(),
        }
        .render()
        .unwrap()
    }
}

// * FurnaceRecipe

pub struct FurnaceRecipe<'a> {
    pub id: &'a str,
    pub tags: Vec<&'a str>,
    pub input: RecipeInputOrOutput<'a>,
    pub output: &'a str,
}
impl<'a> Recipe for FurnaceRecipe<'a> {
    fn serialize(&self) -> String {
        FurnaceRecipeTemplate {
            id: self.id.to_string(),
            tags: format!("{:?}", self.tags),
            input: self.input.serialize(),
            output: self.output.to_string(),
        }
        .render()
        .unwrap()
    }
    fn id(&self) -> String {
        self.id.to_string()
    }
}

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

pub struct ShapelessRecipe<'a> {
    pub id: &'a str,
    pub tags: Vec<&'a str>,
    pub ingredients: Vec<RecipeInputOrOutput<'a>>,
    pub result: RecipeInputOrOutput<'a>,
}
impl<'a> Recipe for ShapelessRecipe<'a> {
    fn serialize(&self) -> String {
        let ingredients: &Vec<RecipeInputOrOutput> = self.ingredients.as_ref();
        ShapelessRecipeTemplate {
            id: self.id.to_string(),
            tags: format!("{:?}", self.tags),
            ingredients: serialize_ingredients(ingredients),
            result: self.result.serialize(),
        }
        .render()
        .unwrap()
    }
    fn id(&self) -> String {
        self.id.to_string()
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

pub struct ShapedRecipe<'a> {
    pub id: &'a str,
    pub tags: Vec<&'a str>,
    pub ingredients: Vec<RecipeInputOrOutput<'a>>,
    pub result: RecipeInputOrOutput<'a>,
    pub pattern: Vec<&'a str>,
}

impl<'a> Recipe for ShapedRecipe<'a> {
    fn serialize(&self) -> String {
        let ingredients: &Vec<RecipeInputOrOutput> = self.ingredients.as_ref();
        ShapedRecipeTemplate {
            id: self.id.to_string(),
            tags: format!("{:?}", self.tags),
            ingredients: serialize_ingredients(ingredients),
            result: self.result.serialize(),
            pattern: format!("{:?}", self.pattern),
        }
        .render()
        .unwrap()
    }
    fn id(&self) -> String {
        self.id.to_string()
    }
}

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
