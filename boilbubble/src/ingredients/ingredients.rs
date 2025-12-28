use turbo::*;

#[turbo::serialize]
#[derive(PartialEq, Copy)]
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
    pub fn new(name: &str) -> Self {
        Self {
            ingredType: IngredientType::Sweet,
            name: name.to_string(),
        }
    }
    pub fn setType(&mut self, typeID: &str) {
        match typeID {
            "Sweet" => self.ingredType = IngredientType::Sweet,
            "Saltly" => self.ingredType = IngredientType::Saltly,
            "Sour" => self.ingredType = IngredientType::Sour,
            "Aromatic" => self.ingredType = IngredientType::Aromatic,
            "Spicy" => self.ingredType = IngredientType::Spicy,
            "Earthy" => self.ingredType = IngredientType::Earthy,
            "Savory" => self.ingredType = IngredientType::Savory,
            "Fruity" => self.ingredType = IngredientType::Fruity,
            "Thick" => self.ingredType = IngredientType::Thick,
            "Chunky" => self.ingredType = IngredientType::Chunky,
            "Heavy" => self.ingredType = IngredientType::Heavy,
            "Light" => self.ingredType = IngredientType::Light,
            _ => (),
        }
    }
}