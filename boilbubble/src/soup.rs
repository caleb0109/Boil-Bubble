use std::vec;

use turbo::*;
use crate::ingredients::{self, ingredients::Ingredient};
use crate::ingredients::IngredientType;
use crate::UIButtons::UIButtons::UIButton;

#[turbo::serialize]
pub struct Soup {
    pub soup: Vec<Ingredient>,
    pub limit: usize,
}

impl Soup {
    pub fn new() -> Self {
        Self {
            soup: Vec::new(),
            limit: 0,
        }
    }
    pub fn addIngredients(&mut self, ingredient: Ingredient) {
        text!("Adding Ingredient: {}", ingredient.name; x = 0, y = 70, color = 0x22406eff);
        println!("Adding Ingredient: {}", ingredient.name);
        if ingredient.name == "empty" {
            return;
        } else if self.soup.len() < self.limit {
            self.soup.push(ingredient);
        } else {
            self.soup.remove(0);
            self.soup.push(ingredient);
        }
    }
    pub fn dumpSoup(&mut self) {
        self.soup = Vec::new();
    }
}