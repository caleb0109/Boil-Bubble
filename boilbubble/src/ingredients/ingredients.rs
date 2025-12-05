use turbo::*;

#[turbo::serialize]
//enum to define different ingredient types
pub enum IngredientType {
    Sweet,
    Saltly,
    Sour,
    Aromatic,
    Spicy,
    Earthy,
    Savory,
    Fruity,
    Thick,
    Chunky,
    Heavy,
    Light
}

#[turbo::serialize]
pub struct Ingredient {
    pub ingredType: IngredientType,
    pub name: String,
}

impl Ingredient {
    pub fn new(ingredType: IngredientType, name: &str) -> Self {
        Self {
            ingredType,
            name: name.to_string(),
        }
    }
}