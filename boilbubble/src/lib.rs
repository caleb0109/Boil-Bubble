mod customer;
mod ingredients;
mod track;
mod UIButtons;
mod soup;

use turbo::*;
use turbo::time::tick;
use track::Track;
use soup::Soup;
use crate::UIButtons::UIButtons::UIButton;
use crate::ingredients::Ingredient;
use crate::ingredients::IngredientType;


#[turbo::game]
struct GameState {
    day: i32,
    timestamp: usize,
    timepass: usize,
    trackList: Track,
    trackPrint: usize,
    uibuttons: [UIButton; 3],
    soup: Soup,
}
impl GameState {
    pub fn new() -> Self {
        // initialize your game state
        Self {      
            day:0,
            timestamp: time::tick(),
            timepass: 0,
            trackList: Track::new(),
            trackPrint: 0,

            uibuttons: [
                UIButton::new("NextDay", (106.0, 115.0, 40.0, 20.0), false),
                UIButton::new("soup", (115.0, 70.0, 20.0, 20.0), false),
                UIButton::new("soupDump", (140.0, 75.0, 8.0, 8.0), false),
            ],
            soup: Soup::new(),
        }
    }
    pub fn update(&mut self) {
        let mut select: (f32,f32) = (0.0,0.0);
        let mut select2: (f32,f32) = (0.0,0.0);
        let m = pointer::world();
        let(mx, my) = m.xy();
        let x = mx as f32;
        let y = my as f32;
        //random checker
        let mut yPos = 0.0;
        //for every 5/6 of a second that pass, the next item on the track will appear
        //is not perfect for sure, but visually works for now
        //will look into further ways to optimize
        if time::tick() % 50 == 0 && self.trackPrint <= 7 && self.day > 0{
            self.trackPrint += 1;
        }
        //for loop to create the track
        for n in 0..self.trackPrint {
            select = self.trackList.ingredPos1[n].0.check(select);
            select2 = self.trackList.ingredPos2[n].0.check(select2);
            //checks if the track item is at the end of the opposite side from start
            if !self.trackList.trackPos1[n].2 {
                //if the track item has yet to reach the max height and is on starting side
                if self.trackList.trackPos1[n].0 <= 248.0 && !self.trackList.trackPos1[n].2{
                    self.trackList.trackPos1[n].0 += 0.625;
                }
                //if track item reaches end of sceen on opposite side
                if self.trackList.trackPos1[n].0 >= 248.0 {
                    self.trackList.trackPos1[n].2 = true;
                }
            }
            //checks if the track item is on the way back to starting side
            if !self.trackList.trackPos2[n].2{
                //if the track item reaches min height and is on starting side
                if self.trackList.trackPos2[n].0 >= 0.0 && !self.trackList.trackPos1[n].2{
                    self.trackList.trackPos2[n].0 -= 0.625;
                }
                //if track item reached min height, now descending and making sure it doesn't go above starting height
                if self.trackList.trackPos2[n].0 <= 2.5{
                    self.trackList.trackPos2[n].2 = true;
                }
            }

            //if ingredient thats being held is hovering over the soup box and the mouse was just released
            //then it will add the ingredient that was being held to the soup and set the
            //ingredient on the track to empty/nothing
            if self.trackList.ingredPos1[n].0.hover(self.trackList.ingredPos1[n].0.hitbox, x, y) && 
               self.uibuttons[1].hover(self.uibuttons[1].hitbox, x, y) && m.just_released(){
                self.soup.addIngredients(self.trackList.ingredPos1[n].1.clone());
                //self.trackList.ingredPos2[n].1.ingredType = crate::ingredients::IngredientType::Empty;
                self.trackList.ingredPos1[n].1.name = "empty".to_string();
                self.trackList.ingredPos1[n].0.action = false;
                
            } else if self.trackList.ingredPos2[n].0.hover(self.trackList.ingredPos2[n].0.hitbox, x, y) && 
               self.uibuttons[1].hover(self.uibuttons[1].hitbox, x, y) && m.just_released(){
                self.soup.addIngredients(self.trackList.ingredPos2[n].1.clone());
                //self.trackList.ingredPos2[n].1.ingredType = crate::ingredients::IngredientType::Empty;
                self.trackList.ingredPos2[n].1.name = "empty".to_string();
                self.trackList.ingredPos2[n].0.action = false;
            }
            //if the ingredient isn't being held, then set its position to the track position
            if !self.trackList.ingredPos1[n].0.action {
                self.trackList.ingredPos1[n].0.hitbox.0 = self.trackList.trackPos1[n].0;
                self.trackList.ingredPos1[n].0.hitbox.1 = self.trackList.trackPos1[n].1;
            }
            if !self.trackList.ingredPos2[n].0.action {
                self.trackList.ingredPos2[n].0.hitbox.0 = self.trackList.trackPos2[n].0;
                self.trackList.ingredPos2[n].0.hitbox.1 = self.trackList.trackPos2[n].1;
            }
            //if the pointer releases the ingredient, ingredient is not active
            
            
            if m.just_released() {
                self.trackList.ingredPos1[n].0.action = false;
                self.trackList.ingredPos2[n].0.action = false;
            }

            
            //if it has a specific name and , then draw rect to see difference
            if self.trackList.ingredPos1[n].1.name == "Sugar" || self.trackList.ingredPos1[n].1.name == "Salt" && !self.trackList.trackPos1[n].2{
                self.trackList.ingredPos1[n].0.tempDraw("Sugar");
            }
            if self.trackList.ingredPos2[n].1.name == "Sugar" || self.trackList.ingredPos2[n].1.name == "Salt" && !self.trackList.trackPos2[n].2{
                self.trackList.ingredPos2[n].0.tempDraw("Sugar");
            }
            if self.trackList.ingredPos1[n].1.name == "Peppers" && !self.trackList.trackPos1[n].2{
                self.trackList.ingredPos1[n].0.tempDraw("Peppers");
            }
            //if the track item reaches the end of the screen, then reset it to start
            if !self.trackList.trackPos1[n].2 {
                circ!(x = self.trackList.trackPos1[n].0, y = self.trackList.trackPos1[n].1, d=8, color = 0x32CD32ff);
            } else if self.trackList.trackPos1[n].2 {
                if self.trackList.ingredPos1[n].1.name == "empty" {
                    self.trackList.ingredPos1[n].1 = self.trackList.ingredientGen();
                    
                }
                self.trackList.trackPos1[n].2 = false;
                self.trackList.trackPos1[n].0 = 0.0;
            }
            if !self.trackList.trackPos2[n].2 {
                circ!(x = self.trackList.trackPos2[n].0, y = self.trackList.trackPos2[n].1, d=8, color = 0x32CD32ff);
            } else if self.trackList.trackPos2[n].2 {
                if self.trackList.ingredPos2[n].1.name == "empty" {
                    self.trackList.ingredPos2[n].1 = self.trackList.ingredientGen();
                }
                self.trackList.trackPos2[n].2 = false;
                self.trackList.trackPos2[n].0 = 250.0;
            }
            yPos += 10.0;
            //text!("ingred: {}", self.trackList.ingredPos1[n].1.name; x = 0, y = yPos);
        }

        let ingredientListTemp = vec![
                    Ingredient::new(crate::ingredients::IngredientType::Sweet, "Sugar"),
                    Ingredient::new(crate::ingredients::IngredientType::Spicy, "Peppers"),
                    Ingredient::new(crate::ingredients::IngredientType::Saltly, "Salt"),
                    Ingredient::new(crate::ingredients::IngredientType::Sour, "Sugar"),
                ];
        //check to see if day continue button is pressed or not
        for n in 0..self.uibuttons.len() {
            let dayPress = self.uibuttons[n].check(select);
                    //if pressed, goes to next day, resets all track positions, empties soup, and sets soup limit
                    //resetting will all occur here when going to next day for now
                    //eventually will have file reader to load in new ingredient lists, customer orders, etc.
            if self.uibuttons[n].action && self.uibuttons[n].text == "NextDay" {
                self.day += 1;
                self.uibuttons[0].action = false;
                self.trackPrint = 0;
                self.soup.limit = 4;
                self.soup.soup = Vec::new();
                self.trackList.dayIngredients(ingredientListTemp.clone());
                for n in 0..8 {
                    self.trackList.trackPos1[n] = (0.0,100.0,false);
                    self.trackList.trackPos2[n] = (250.0,30.0,false);
                    self.trackList.ingredPos1[n].0.hitbox.0 = 0.0;
                    self.trackList.ingredPos1[n].0.hitbox.1 = 100.0;
                    self.trackList.ingredPos2[n].0.hitbox.0 = 250.0;
                    self.trackList.ingredPos2[n].0.hitbox.1 = 30.0;

                    self.trackList.ingredPos1[n].1 = Ingredient::new(IngredientType::Sweet, "empty");
                    self.trackList.ingredPos2[n].1 = Ingredient::new(IngredientType::Sweet, "empty");
                }   
            } else if self.uibuttons[n].action && self.uibuttons[n].text == "soupDump" {
                self.soup.dumpSoup();
                self.uibuttons[2].action = false;
            }
            self.uibuttons[n].tempDraw("ui");
        }

        text!("Soup {}", self.soup.soup.len(); x = 0, y = 0);
        text!("Day: {}", self.day; x = 0, y = 10);
         for n in 0..self.soup.soup.len() {
            text!("Soup: {}", self.soup.soup[n].name; x = 0, y = 20);
         }
        

    }


}