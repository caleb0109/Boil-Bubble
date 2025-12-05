use turbo::*;
use crate::ingredients::{self, ingredients::Ingredient};
use crate::ingredients::IngredientType;
use crate::UIButtons::UIButtons::UIButton;

#[turbo::serialize]
pub struct Track {
    //data of track and ingredients on track
    pub trackPos: [(f32,f32,bool,bool); 10],
    pub ingredPos: [(UIButton, Ingredient); 10],
    //Ingredient list for the day
    pub ingredList: Vec<Ingredient>,
}

impl Track {
    pub fn new() -> Self {
        Self {
            //currTrack: Vec<Ingredient>, list: Vec<Ingredient>
            // trackList: currTrack,
            // ingredList: list,
            trackPos: [
                //x Position, y Position, check if reached lowest or highest point in track, check if it reached end of screen
                (0.0,70.0,false,false),
                (0.0,70.0,false,false),
                (0.0,70.0,false,false),
                (0.0,70.0,false,false),
                (0.0,70.0,false,false),
                (0.0,70.0,false,false),
                (0.0,70.0,false,false),
                (0.0,70.0,false,false),
                (0.0,70.0,false,false),
                (0.0,70.0,false,false)
            ],
            ingredPos: [
                //x Position, y Position, ingredient on the track
                (UIButton::new("ing", (0.0, 70.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (0.0, 70.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "Sugar")),
                (UIButton::new("ing", (0.0, 70.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (0.0, 70.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (0.0, 70.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (0.0, 70.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (0.0, 70.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "Sugar")),
                (UIButton::new("ing", (0.0, 70.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "Sugar")),
                (UIButton::new("ing", (0.0, 70.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty")),
                (UIButton::new("ing", (0.0, 70.0,10.0,10.0), false), 
                Ingredient::new(IngredientType::Sweet, "empty"))
            ],
            ingredList: Vec::new(),
        }
    }

    pub fn dayIngredients(&self) {

    }

    pub fn ingredientGen(&self) {

    }
}