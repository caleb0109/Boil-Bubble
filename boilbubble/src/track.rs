use turbo::*;
use crate::ingredients::ingredients::Ingredient;
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
            //lower track
            trackPos1: [
                (0.0,206.0,false),
                (0.0,206.0,false),
                (0.0,206.0,false),
                (0.0,206.0,false),
                (0.0,206.0,false),
                (0.0,206.0,false),
                (0.0,206.0,false),
                (0.0,206.0,false),
            ],
            //upper track
            trackPos2: [
                (510.0,44.0,false),
                (510.0,44.0,false),
                (510.0,44.0,false),
                (510.0,44.0,false),
                (510.0,44.0,false),
                (510.0,44.0,false),
                (510.0,44.0,false),
                (510.0,44.0,false),
            ],
            //x Position, y Position, ingredient on the track
            ingredPos1: [
                (UIButton::new("ing", (0.0,206.0,50.0,30.0), false), 
                Ingredient::new( "empty")),
                (UIButton::new("ing", (0.0,206.0,50.0,30.0), false), 
                Ingredient::new("empty")),
                (UIButton::new("ing", (0.0,206.0,50.0,30.0), false), 
                Ingredient::new("empty")),
                (UIButton::new("ing", (0.0,206.0,50.0,30.0), false), 
                Ingredient::new("empty")),
                (UIButton::new("ing", (0.0,206.0,50.0,30.0), false), 
                Ingredient::new("empty")),
                (UIButton::new("ing", (0.0,206.0,50.0,30.0), false), 
                Ingredient::new("empty")),
                (UIButton::new("ing", (0.0,206.0,50.0,30.0), false), 
                Ingredient::new("empty")),
                (UIButton::new("ing", (0.0,206.0,50.0,30.0), false), 
                Ingredient::new("empty")),
            ],
            //lower track
            ingredPos2: [
                (UIButton::new("ing", (510.0,44.0,50.0,30.0), false), 
                Ingredient::new("empty")),
                (UIButton::new("ing", (510.0,44.0,50.0,30.0), false), 
                Ingredient::new("empty")),
                (UIButton::new("ing", (510.0,44.0,50.0,30.0), false), 
                Ingredient::new("empty")),
                (UIButton::new("ing", (510.0,44.0,50.0,30.0), false), 
                Ingredient::new("empty")),
                (UIButton::new("ing", (510.0,44.0,50.0,30.0), false), 
                Ingredient::new("empty")),
                (UIButton::new("ing", (510.0,44.0,50.0,30.0), false), 
                Ingredient::new("empty")),
                (UIButton::new("ing", (510.0,44.0,50.0,30.0), false), 
                Ingredient::new("empty")),
                (UIButton::new("ing", (510.0,44.0,50.0,30.0), false), 
                Ingredient::new("empty")),
            ],
            ingredList: vec![
                Ingredient::new("sugar"),
                Ingredient::new("chocolate"),
                Ingredient::new("salt"),
                Ingredient::new("olives"),
                Ingredient::new("lemon"),
                Ingredient::new("cranberries"),
                Ingredient::new("peppers"),
                Ingredient::new("ginger"),
                Ingredient::new("mushrooms"),
                Ingredient::new("carrots"),
                Ingredient::new("beef"),
                Ingredient::new("fish"),
                Ingredient::new("apples"),
                Ingredient::new("watermelon"),
                Ingredient::new("cheese"),
                Ingredient::new("flour"),
            ],
        }
    }

    pub fn dayIngredients(&mut self, ingredients: Vec<Ingredient>) {
        self.ingredList = ingredients;
    }

    pub fn ingredientGen(&self) -> Ingredient{
        if self.ingredList.is_empty() {
            return Ingredient::new("empty");
        }
        //text!("gen", x = 0, y = 60);
        let num: u32 = random::between(0,self.ingredList.len() as u32 -1);
        let choice = num as usize;

        match random::between(0,4) {
            0 => {return Ingredient::new("empty")}
            _ => {return self.ingredList[choice].clone()}
        }
        
    }
}