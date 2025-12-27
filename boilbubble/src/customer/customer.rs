use turbo::*;
use crate::ingredients::{self, ingredients::Ingredient};
use crate::ingredients::IngredientType;

#[turbo::serialize]
pub struct Customer {
    pub cusName: String,
    pub orderDesc: String,
    pub order: Vec<Ingredient>,
    pub patienceTime: usize,
    pub score: i32,
}

impl Customer {
    pub fn new(name: &str, orderDesc: &str, order: Vec<Ingredient>) -> Self {
        Self {
            cusName: name.to_string(),
            orderDesc: orderDesc.to_string(),
            order: order,
            patienceTime: 0,
            score: 0,
        }
    }

    pub fn createOrder(&self) {

    }

    pub fn ticket(&self) {
        
    }

    pub fn serverSoup(&mut self, soup: &Vec<Ingredient>) -> i32 {
        for (i, ingredients) in self.order.iter().enumerate() {
            if ingredients.ingredType == soup[i].ingredType {
                self.score += 1;
            }
        }
        return self.score;
    }

    //score calculation based on how many correct ingredients and patience time left
    //score split:
    // - ingredients correct: 80%
    // - patience time left: 20%
    pub fn calculateScore(&mut self) -> i32 {
        return self.score;
    }
}