use turbo::*;
use crate::ingredients::{self, ingredients::Ingredient};
use crate::ingredients::IngredientType;
use crate::UIButtons::UIButtons::UIButton;

#[turbo::serialize]
pub struct Track {
    //data of track and ingredients on track
    pub trackPos1: [(f32,f32,bool); 8],
    pub trackPos2: [(f32,f32,bool); 8],
    pub ingredPos1: [(UIButton, Ingredient); 8],
    pub ingredPos2: [(UIButton, Ingredient); 8],
    //Ingredient list for the day
    pub ingredList: Vec<Ingredient>,
}

impl Track {
    pub fn new() -> Self {
        Self {
            //x Position, y Position, check if reached lowest or highest point in track, check if it reached end of screen
            trackPos1: [
                (0.0,100.0,false),
                (0.0,100.0,false),
                (0.0,100.0,false),
                (0.0,100.0,false),
                (0.0,100.0,false),
                (0.0,100.0,false),
                (0.0,100.0,false),
                (0.0,100.0,false),
            ],
            //lower track
            trackPos2: [
                (250.0,30.0,false),
                (250.0,30.0,false),
                (250.0,30.0,false),
                (250.0,30.0,false),
                (250.0,30.0,false),
                (250.0,30.0,false),
                (250.0,30.0,false),
                (250.0,30.0,false),
            ],
            //x Position, y Position, ingredient on the track
            ingredPos1: [
                (UIButton::new("ing", (0.0,100.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (0.0,100.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "Sugar")),
                (UIButton::new("ing", (0.0,100.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (0.0,100.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (0.0,100.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (0.0,100.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (0.0,100.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "Sugar")),
                (UIButton::new("ing", (0.0,100.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "Sugar")),
            ],
            //lower track
            ingredPos2: [
                (UIButton::new("ing", (250.0,30.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (250.0,30.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (250.0,30.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (250.0,30.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "Sugar")),
                (UIButton::new("ing", (250.0,30.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (250.0,30.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (250.0,30.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (250.0,30.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "Sugar")),
            ],
            ingredList: Vec::new(),
        }
    }

    pub fn dayIngredients(&self) {

    }

    pub fn ingredientGen(&self) {

    }
}