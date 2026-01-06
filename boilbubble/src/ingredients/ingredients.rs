use turbo::*;

#[turbo::serialize]
#[derive(PartialEq, Copy)]
//enum to define different ingredient types
pub enum IngredientType {
    Sweet,
    Saltly,
    Sour,
    Spicy,
    Earthy,
    Savory,
    Fruity,
    Thick,
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
            "sugar" => self.ingredType = IngredientType::Sweet,
            "chocolate" => self.ingredType = IngredientType::Sweet,
            "salt" => self.ingredType = IngredientType::Saltly,
            "olives" => self.ingredType = IngredientType::Saltly,
            "lemon" => self.ingredType = IngredientType::Sour,
            "cranberries" => self.ingredType = IngredientType::Sour,
            "peppers" => self.ingredType = IngredientType::Spicy,
            "ginger" => self.ingredType = IngredientType::Spicy,
            "mushroom" => self.ingredType = IngredientType::Earthy,
            "carrot" => self.ingredType = IngredientType::Earthy,
            "meat" => self.ingredType = IngredientType::Savory,
            "fish" => self.ingredType = IngredientType::Savory,
            "apple" => self.ingredType = IngredientType::Fruity,
            "watermelon" => self.ingredType = IngredientType::Fruity,
            "cheese" => self.ingredType = IngredientType::Thick,
            "flour" => self.ingredType = IngredientType::Thick,
            _ => (),
        }
    }
}