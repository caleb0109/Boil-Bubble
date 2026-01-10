use turbo::*;
use crate::ingredients::{self, ingredients::Ingredient};
use crate::ingredients::IngredientType;

#[turbo::serialize]
pub struct Customer {
    pub cusName: String,
    pub orderDesc: String,
    pub order: Vec<Ingredient>,
    //pub patienceTime: usize,
    pub score: f32,
    pub total: i32,
}

impl Customer {
    pub fn new(name: &str, orderDesc: &str, order: Vec<Ingredient>) -> Self {
        Self {
            cusName: name.to_string(),
            orderDesc: orderDesc.to_string(),
            order: order,
            //patienceTime: 15,
            score: 0.0,
            total: 0,
        }
    }

    pub fn createOrder(&mut self, timer: usize, day: i32) {      
        text!(&self.orderDesc, x = 67, y = 268, font = "TENPIXELS", color = 0x2d1e1eff);
        //self.patienceTime = 15 - timer;

        let time = timer as f32;
        let mut gap = 0.0;

        if day > 0 && day <= 2 { gap = 6.0; }
        else if day > 2 && day <= 4{ gap = 5.0; }
        else if day > 4 && day <= 7 { gap = 4.5; }
        else if day >= 8 { gap = 4.0; }

        if time < gap {
            let cusSprite = format!("customers#{}", &self.cusName);
            sprite!(&cusSprite, x = 0, y  = 261);
        } else if time >= gap && time < gap * 2.0 {
            let cusSprite = format!("customers_patience1#{}", &self.cusName);
            sprite!(&cusSprite, x = 0, y  = 261);
        } else if time >= gap * 2.0 && time < gap * 3.0 {
            let cusSprite = format!("customers_patience2#{}", &self.cusName);
            sprite!(&cusSprite, x = 0, y  = 261);
        } else if time >= gap * 3.0 && time < gap * 4.0 {
            let cusSprite = format!("customers_patience3#{}", &self.cusName);
            sprite!(&cusSprite, x = 0, y  = 261);
        }

    }

    pub fn serveSoup(&mut self, soup: &Vec<Ingredient>) -> f32 {
        if soup.len() == 0 {
            self.score = 0.0;
            return self.score;
        }
        let mut checked: Vec<bool> = vec![false; self.order.len()];
        for n in 0..self.order.len() {
            for m in 0..soup.len() {
                if !checked[m] {
                    if self.order[n].ingredType == soup[m].ingredType {
                        checked[m] = true;
                        self.score += 1.0;
                    }
                } 
            }
        }
        return self.score;
    }

    //score calculation based on how many correct ingredients and patience time left
    //score split:
    // - ingredients correct: 80%
    // - patience time left: 20%
    pub fn calculateScore(&mut self, cusTimer: usize, cusLim: usize) -> i32 {
        let ingredScore = (self.score / self.order.len() as f32) * 80.0;
        let timeScore = ((cusLim - cusTimer) as f32 / cusLim as f32) * 20.0;
        self.total = (ingredScore + timeScore) as i32;
        return self.total as i32;
    }

    pub fn drawScoreReaction(&mut self) -> bool {
        let anim = animation::get("reaction");
        
        anim.set_fill_forwards(true);

        let percentage = self.score / self.order.len() as f32;
        if percentage <= 0.3 {           
            anim.use_sprite("sadcustomer");
            anim.set_repeat(1);
            //sprite!(animation_key = "reaction", default_sprite = "emptycustomer", x = 118, y = 136);
        } else if percentage > 0.3 && percentage <= 0.7 {
            anim.use_sprite("neutralcustomer");
            anim.set_repeat(1);
            //sprite!(animation_key = "reaction", default_sprite = "emptycustomer", x = 118, y = 136);
        } else if percentage > 0.7 {
            anim.use_sprite("happycustomer");
            anim.set_repeat(1);
            //sprite!(animation_key = "reaction", default_sprite = "emptycustomer", x = 118, y = 136);
        } 
        sprite!(animation_key = "reaction", x = 118, y = 136);
        return anim.done();
    }
}