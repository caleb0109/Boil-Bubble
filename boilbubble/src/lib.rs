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
    tList: Track,
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
            tList: Track::new(),
            trackPrint: 0,

            uibuttons: [
                UIButton::new("NextDay", (106.0, 115.0, 40.0, 20.0), false),
                UIButton::new("soup", (145.0, 148.0, 20.0, 20.0), false),
                UIButton::new("soupDump", (140.0, 75.0, 8.0, 8.0), false),
            ],
            soup: Soup::new(),
        }
    }
    pub fn update(&mut self) {
        //sprites that cannot be interacted with
        sprite!("background", x= 0, y = 0);
        sprite!("cauldron", x = 145, y = 148);
        

        self.uibuttons[1].draw();

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
            select = self.tList.ingredPos1[n].0.check(select);
            select2 = self.tList.ingredPos2[n].0.check(select2);
            //checks if the track item is at the end of the opposite side from start
            if !self.tList.trackPos1[n].2 {
                //if the track item has yet to reach the max height and is on starting side
                if self.tList.trackPos1[n].0 <= 510.0 && !self.tList.trackPos1[n].2{
                    self.tList.trackPos1[n].0 += 1.0;
                }
                //if track item reaches end of sceen on opposite side
                if self.tList.trackPos1[n].0 >= 510.0 {
                    self.tList.trackPos1[n].2 = true;
                }
            }
            //checks if the track item is on the way back to starting side
            if !self.tList.trackPos2[n].2{
                //if the track item reaches min height and is on starting side
                if self.tList.trackPos2[n].0 >= 0.0 && !self.tList.trackPos1[n].2{
                    self.tList.trackPos2[n].0 -= 0.625;
                }
                //if track item reached min height, now descending and making sure it doesn't go above starting height
                if self.tList.trackPos2[n].0 <= 2.5{
                    self.tList.trackPos2[n].2 = true;
                }
            }


            //if ingredient thats being held is hovering over the soup box and the mouse was just released
            //then it will add the ingredient that was being held to the soup and set the
            //ingredient on the track to empty/nothing
            if self.tList.ingredPos1[n].0.hover(self.tList.ingredPos1[n].0.hitbox, x, y) && 
               self.uibuttons[1].hover(self.uibuttons[1].hitbox, x, y) && m.just_released(){
                self.soup.addIngredients(self.tList.ingredPos1[n].1.clone());
                //self.tList.ingredPos2[n].1.ingredType = crate::ingredients::IngredientType::Empty;
                self.tList.ingredPos1[n].1.name = "empty".to_string();
                self.tList.ingredPos1[n].0.action = false;
                
            } else if self.tList.ingredPos2[n].0.hover(self.tList.ingredPos2[n].0.hitbox, x, y) && 
               self.uibuttons[1].hover(self.uibuttons[1].hitbox, x, y) && m.just_released(){
                self.soup.addIngredients(self.tList.ingredPos2[n].1.clone());
                //self.tList.ingredPos2[n].1.ingredType = crate::ingredients::IngredientType::Empty;
                self.tList.ingredPos2[n].1.name = "empty".to_string();
                self.tList.ingredPos2[n].0.action = false;
            }
            //if the ingredient isn't being held, then set its position to the track position
            if !self.tList.ingredPos1[n].0.action {
                self.tList.ingredPos1[n].0.hitbox.0 = self.tList.trackPos1[n].0;
                self.tList.ingredPos1[n].0.hitbox.1 = self.tList.trackPos1[n].1;
            }
            if !self.tList.ingredPos2[n].0.action {
                self.tList.ingredPos2[n].0.hitbox.0 = self.tList.trackPos2[n].0;
                self.tList.ingredPos2[n].0.hitbox.1 = self.tList.trackPos2[n].1;
            }
            //if the pointer releases the ingredient, ingredient is not active
            
            
            if m.just_released() {
                self.tList.ingredPos1[n].0.action = false;
                self.tList.ingredPos2[n].0.action = false;
            }

            
            //if it has a specific name and , then draw rect to see difference
            if self.tList.ingredPos1[n].1.name == "Sugar" || self.tList.ingredPos1[n].1.name == "Salt" && !self.tList.trackPos1[n].2{
                self.tList.ingredPos1[n].0.tempDraw("Sugar");
            }
            if self.tList.ingredPos2[n].1.name == "Sugar" || self.tList.ingredPos2[n].1.name == "Salt" && !self.tList.trackPos2[n].2{
                self.tList.ingredPos2[n].0.tempDraw("Sugar");
            }
            if self.tList.ingredPos1[n].1.name == "Peppers" && !self.tList.trackPos1[n].2{
                self.tList.ingredPos1[n].0.tempDraw("Peppers");
            }
            //if the track item reaches the end of the screen, then reset it to start
            if !self.tList.trackPos1[n].2 {
                sprite!("bowl", x = self.tList.trackPos1[n].0, y = self.tList.trackPos1[n].1);
                //circ!(x = self.tList.trackPos1[n].0, y = self.tList.trackPos1[n].1, d=8, color = 0x32CD32ff);
            } else if self.tList.trackPos1[n].2 {
                if self.tList.ingredPos1[n].1.name == "empty" {
                    self.tList.ingredPos1[n].1 = self.tList.ingredientGen();
                    
                }
                self.tList.trackPos1[n].2 = false;
                self.tList.trackPos1[n].0 = 0.0;
            }
            if !self.tList.trackPos2[n].2 {
                sprite!("bowl", x = self.tList.trackPos2[n].0, y = self.tList.trackPos2[n].1);
                //circ!(x = self.tList.trackPos2[n].0, y = self.tList.trackPos2[n].1, d=8, color = 0x32CD32ff);
            } else if self.tList.trackPos2[n].2 {
                if self.tList.ingredPos2[n].1.name == "empty" {
                    self.tList.ingredPos2[n].1 = self.tList.ingredientGen();
                }
                self.tList.trackPos2[n].2 = false;
                self.tList.trackPos2[n].0 = 510.0;
            }
            yPos += 10.0;
            //text!("ingred: {}", self.tList.ingredPos1[n].1.name; x = 0, y = yPos);
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
                self.tList.dayIngredients(ingredientListTemp.clone());
                for n in 0..8 {
                    self.tList.trackPos1[n] = (0.0,206.0,false);
                    self.tList.trackPos2[n] = (510.0,44.0,false);
                    self.tList.ingredPos1[n].0.hitbox.0 = 0.0;
                    self.tList.ingredPos1[n].0.hitbox.1 = 206.0;
                    self.tList.ingredPos2[n].0.hitbox.0 = 510.0;
                    self.tList.ingredPos2[n].0.hitbox.1 = 44.0;

                    self.tList.ingredPos1[n].1 = Ingredient::new(IngredientType::Sweet, "empty");
                    self.tList.ingredPos2[n].1 = Ingredient::new(IngredientType::Sweet, "empty");
                }   
            } else if self.uibuttons[n].action && self.uibuttons[n].text == "soupDump" {
                self.soup.dumpSoup();
                self.uibuttons[2].action = false;
            }
            self.uibuttons[n].tempDraw("ui");
        }

        sprite!("cat", x = 181, y =65);
        
        //UI
        sprite!("borders", x = 0, y = 0);

        text!("Soup {}", self.soup.soup.len(); font = "TENPIXELS", x = 0, y = 8);
        text!("Day: {}", self.day; font = "TENPIXELS", x = 60, y = 8);
         for n in 0..self.soup.soup.len() {
            text!("Soup: {}", self.soup.soup[n].name; x = 0, y = 8);
         }
        

    }


}