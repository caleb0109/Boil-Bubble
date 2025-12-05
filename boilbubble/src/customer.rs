use turbo::*;
use crate::ingredients::{self, ingredients::Ingredient};

#[turbo::serialize]
pub struct Customer {
    pub cusName: String,
    pub orderDesc: String,
    pub order: Vec<Ingredient>,
    pub patienceTime: usize,
}

impl Customer {
    pub fn new(name: &str, orderDesc: &str, order: Vec<Ingredient>) -> Self {
        Self {
            cusName: name.to_string(),
            orderDesc: orderDesc.to_string(),
            order: order,
            patienceTime: 0,
        }
    }

    pub fn createOrder(&self) {

    }

    pub fn ticket(&self) {
        
    }
}