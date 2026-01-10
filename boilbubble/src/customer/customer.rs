use turbo::*;
use crate::ingredients::{self, ingredients::Ingredient};
use crate::ingredients::IngredientType;

#[turbo::serialize]
pub struct Customer {
    pub cusName: String,
    pub orderDesc: String,
    pub order: Vec<Ingredient>,
    //pub patienceTime: usize,
    pub score: i32,
}

impl Customer {
    pub fn new(name: &str, orderDesc: &str, order: Vec<Ingredient>) -> Self {
        Self {
            cusName: name.to_string(),
            orderDesc: orderDesc.to_string(),
            order: order,
            //patienceTime: 15,
            score: 0,
        }
    }

    pub fn createOrder(&mut self, timer: usize) {      
        text!(&self.orderDesc, x = 67, y = 268, font = "TENPIXELS", color = 0x2d1e1eff);
        //self.patienceTime = 15 - timer;

        if timer <= 10 {
            let cusSprite = format!("customers#{}", &self.cusName);
            sprite!(&cusSprite, x = 0, y  = 261);
        } else if timer > 10 && timer <= 15 {
            let cusSprite = format!("customers_patience1#{}", &self.cusName);
            sprite!(&cusSprite, x = 0, y  = 261);
        } else if timer > 15 && timer <= 20 {
            let cusSprite = format!("customers_patience2#{}", &self.cusName);
            sprite!(&cusSprite, x = 0, y  = 261);
        } else if timer > 20 && timer <= 25 {
            let cusSprite = format!("customers_patience3#{}", &self.cusName);
            sprite!(&cusSprite, x = 0, y  = 261);
        }

    }

    pub fn serveSoup(&mut self, soup: &Vec<Ingredient>) -> i32 {
        if soup.len() == 0 {
            self.score = 0;
            return self.score;
        }
        let mut matches = 0;
        let mut checked: Vec<bool> = vec![false; self.order.len()];
        for n in 0..self.order.len() {
            for m in 0..soup.len() {
                if !checked[m] {
                    if self.order[n].ingredType == soup[m].ingredType {
                        checked[m] = true;
                        matches += 1;
                    }
                } 
            }
        }
        let total = self.order.len() * soup.len();
        self.score = ((matches / total) * 100) as i32;

        return self.score;
    }

    //score calculation based on how many correct ingredients and patience time left
    //score split:
    // - ingredients correct: 80%
    // - patience time left: 20%
    pub fn calculateScore(&mut self, cusTimer: usize) -> i32 {
        let ingredScore = (self.score as f32 / self.order.len() as f32) * 80.0;
        let timeScore = ((15 - cusTimer) as f32 / 15.0) * 20.0;
        return (ingredScore + timeScore) as i32;
    }

    pub fn drawScoreReaction(&mut self) {
        let anim = animation::get("customer");
        if self.score > 1 {
            anim.use_sprite("sadcustomer");
        }
    }
}