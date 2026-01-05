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
            patienceTime: 15,
            score: 0,
        }
    }

<<<<<<< Updated upstream
<<<<<<< Updated upstream
<<<<<<< Updated upstream
=======
=======
>>>>>>> Stashed changes
=======
>>>>>>> Stashed changes
    pub fn createOrder(&self) {      
        text!(&self.orderDesc, x = 67, y = 268, font = "TENPIXELS");

        if self.patienceTime > 10 {
            let cusSprite = format!("customers#{}", &self.cusName);
            sprite!(&cusSprite, x = 0, y  = 261);
        } else if self.patienceTime < 10 && self.patienceTime > 7 {
            let cusSprite = format!("customers_patience1#{}", &self.cusName);
            sprite!(&cusSprite, x = 0, y  = 261);
        } else if self.patienceTime < 7 && self.patienceTime > 5 {
            let cusSprite = format!("customers_patience2#{}", &self.cusName);
            sprite!(&cusSprite, x = 0, y  = 261);
        } else if self.patienceTime < 5 && self.patienceTime > 3 {
            let cusSprite = format!("customers_patience3#{}", &self.cusName);
            sprite!(&cusSprite, x = 0, y  = 261);
        }

    }

>>>>>>> Stashed changes
    pub fn ticket(&self) {
        
    }

    pub fn serveSoup(&mut self, soup: &Vec<Ingredient>) -> i32 {
        for (i, ingredients) in self.order.iter().enumerate() {
            if ingredients.ingredType == soup[i].ingredType {
                self.score += 1;
            }
        }
        return self.score;
    }
    
    //checks if the soup in the pot is the same as the customer's order
    pub fn soupCheck(&mut self, soup: &Vec<Ingredient>) -> bool {
        if self.order.len() != soup.len() {
            return false;
        }

        let mut check = true;
        for (i, ingredients) in self.order.iter().enumerate() {
            if ingredients.ingredType != soup[i].ingredType {
                check = false;
            }
        }
        return check;
    }

    //score calculation based on how many correct ingredients and patience time left
    //score split:
    // - ingredients correct: 80%
    // - patience time left: 20%
    pub fn calculateScore(&mut self) -> i32 {
        return self.score;
    }
}