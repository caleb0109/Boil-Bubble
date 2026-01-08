mod ingredients;
mod track;
mod UIButtons;
mod soup;
mod reader;
mod customer;

use turbo::*;
use turbo::time::tick;
use track::Track;
use soup::Soup;
use crate::UIButtons::UIButtons::UIButton;
use crate::ingredients::Ingredient;
use crate::ingredients::IngredientType;
use crate::customer::Customer;


#[turbo::game]
struct GameState {
    day: i32,
    timestamp: usize,
    timepass: usize,
    tList: Track,
    trackPrint: usize,
    ingredHold: bool,
    ingredCheck: usize,
    uibuttons: [UIButton; 6],
    soup: Soup,
    reader: reader::Reader,
    currCus: usize,
    cusCheck: bool,
    cameraPos: (i32,i32),
    timer: usize,
    cusTimer: usize,
    tutorial: usize,
    timeStamp: usize,
    totalScore: i32,
    startDay: bool,
    checked: Vec<bool>,
    endScreen: bool,
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
            ingredHold: false,
            ingredCheck: 0,
            uibuttons: [
                UIButton::new("start", (200.0, 230.0, 100.0, 20.0), false),
                UIButton::new("soup", (145.0, 148.0, 210.0, 50.0), false),
                UIButton::new("soupDump", (140.0, 75.0, 8.0, 8.0), false),
                UIButton::new("start", (582.0, 174.0, 100.0, 20.0), false),
                UIButton::new("continue", (195.0,230.0, 100.0, 20.0), false),
                UIButton::new("serve", (195.0,230.0, 100.0, 20.0), false),
            ],
            soup: Soup::new(),
            reader: reader::Reader::new(),
            currCus: 0,
            cusCheck: false,
            cameraPos: (765,143),
            timer: 0,
            cusTimer: 0,
            tutorial: 0,
            timeStamp: time::tick(),
            totalScore: 0,
            startDay: false,
            checked: vec![false; 8],
            endScreen: false,
        }
    }
    pub fn update(&mut self) {
        //sprites that cannot be interacted with
        sprite!("titlescreen", x = 510, y = 0);
        sprite!("background", x= 0, y = 0);
        sprite!("cauldron", x = 145, y = 148);
        sprite!("bowls_lowertrack", x = 0, y = 0);
        sprite!("bowls_uppertrack", x = 0, y = 0);
        sprite!("cat", x = 181, y =65);
        
        //UI
        sprite!("list", x = 4, y = 88);
        sprite!("borders", x = 0, y = 0);
        sprite!("customer_speech", x = 59, y = 266);

        if !audio::is_playing("boil_and_bubble") {
            audio::play("boil_and_bubble");
            audio::set_volume("boil_and_bubble", 0.1);
        }

        self.uibuttons[1].draw();
       
        //timer
        let timer_anim = animation::get("timer");
        timer_anim.use_sprite("timer");
        sprite!(animation_key = "timer", x = 444, y = 80);
        timer_anim.set_speed(0.4);

        camera::set_xy(self.cameraPos.0, self.cameraPos.1);

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
        if time::tick() % 64 == 0 && self.trackPrint <= 7 && self.day > 0{
            self.trackPrint += 1;
        }

        if time::tick() % 60 == 0 && self.day > 0 && !self.cusCheck && self.timer < 60{
            self.timer += 1;
        }

        if time::tick() % 60 == 0 && self.day > 0 && !self.cusCheck && self.cusTimer <= 15{
            self.cusTimer += 1;
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
                    self.tList.trackPos2[n].0 -= 1.0;
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
                audio::play("splash");
                audio::set_volume("splash", 0.1);
                //self.tList.ingredPos2[n].1.ingredType = crate::ingredients::IngredientType::Empty;
                self.tList.ingredPos1[n].1.name = "empty".to_string();
                self.tList.ingredPos1[n].1.setType("empty");
                self.tList.ingredPos1[n].0.action = false;
                self.ingredHold = false;
            } else if self.tList.ingredPos2[n].0.hover(self.tList.ingredPos2[n].0.hitbox, x, y) && 
               self.uibuttons[1].hover(self.uibuttons[1].hitbox, x, y) && m.just_released(){
                self.soup.addIngredients(self.tList.ingredPos2[n].1.clone());
                audio::play("splash");
                audio::set_volume("splash", 0.1);
                //self.tList.ingredPos2[n].1.ingredType = crate::ingredients::IngredientType::Empty;
                self.tList.ingredPos2[n].1.name = "empty".to_string();
                self.tList.ingredPos2[n].1.setType("empty");
                self.tList.ingredPos2[n].0.action = false;
                self.ingredHold = false;
            }

            //if the player isn't holding an ingredient, but is doing the action to do so
            //allow them to hold that specific ingredient. Makes the other ingredient in the
            //same track number not holdable to make the distinction
            if self.tList.ingredPos1[n].0.action && !self.ingredHold{
                self.tList.ingredPos2[n].0.action = false;
                self.ingredHold = true;
                self.ingredCheck = n;
            } else if self.tList.ingredPos2[n].0.action && !self.ingredHold{
                self.tList.ingredPos1[n].0.action = false;
                self.ingredHold = true;
                self.ingredCheck = n;
            }
            
            //if the player is already holding an ingredient, but is hovering over/trying to hold
            //other moving ingredients that is not the ingredient thats being held
            //OR if the player is not holding an ingredient at all
            //OR if the player is holding an ingredient, but isn't interacting with an ingredient
            //then sets that specific ingredient's position back to the track position while making sure
            //that ingredient cannot be held/interacted with because the player is already holding one
            if self.tList.ingredPos1[n].0.action && self.ingredHold && self.ingredCheck != n 
            || !self.ingredHold 
            || !self.tList.ingredPos1[n].0.action && self.ingredHold{
                self.tList.ingredPos1[n].0.action = false;
                self.tList.ingredPos1[n].0.hitbox.0 = self.tList.trackPos1[n].0;
                self.tList.ingredPos1[n].0.hitbox.1 = self.tList.trackPos1[n].1;
            }
            //ditto for the second track
            if self.tList.ingredPos2[n].0.action && self.ingredHold && self.ingredCheck != n 
            || !self.ingredHold 
            || !self.tList.ingredPos2[n].0.action && self.ingredHold{
                self.tList.ingredPos2[n].0.action = false;
                self.tList.ingredPos2[n].0.hitbox.0 = self.tList.trackPos2[n].0;
                self.tList.ingredPos2[n].0.hitbox.1 = self.tList.trackPos2[n].1;
            }

            
            //if the pointer releases the ingredient, ingredient is not active
            if m.just_released() {
                self.ingredHold = false;
                self.tList.ingredPos1[n].0.action = false;
                self.tList.ingredPos2[n].0.action = false;
            }


            if self.startDay && self.checked[n] == false {
                self.tList.ingredPos1[n].1 = self.tList.ingredientGen();
                self.tList.ingredPos2[n].1 = self.tList.ingredientGen();
                self.checked[n] = true;
            }
            //if the track item reaches the end of the screen, then reset it to start
            //if the track item is not at the end of the screen, draws the bowl and ingredient
            if !self.tList.trackPos1[n].2 {
                sprite!("bowl", x = self.tList.ingredPos1[n].0.hitbox.0, y = self.tList.ingredPos1[n].0.hitbox.1);
                sprite!(&self.tList.ingredPos1[n].1.name, x = self.tList.ingredPos1[n].0.hitbox.0, y = self.tList.ingredPos1[n].0.hitbox.1 - 11.0);
            } else if self.tList.trackPos1[n].2 {
                self.startDay = false;
                self.tList.ingredPos1[n].1 = self.tList.ingredientGen();
                self.tList.trackPos1[n].2 = false;
                self.tList.trackPos1[n].0 = 0.0;
            }
            if !self.tList.trackPos2[n].2 {
                sprite!("bowl", x = self.tList.ingredPos2[n].0.hitbox.0, y = self.tList.ingredPos2[n].0.hitbox.1);
                sprite!(&self.tList.ingredPos2[n].1.name, x = self.tList.ingredPos2[n].0.hitbox.0, y = self.tList.ingredPos2[n].0.hitbox.1 - 11.0);
            } else if self.tList.trackPos2[n].2 {
                self.startDay = false;
                self.tList.ingredPos2[n].1 = self.tList.ingredientGen();
                self.tList.trackPos2[n].2 = false;
                self.tList.trackPos2[n].0 = 510.0;
            }
            //yPos += 10.0;
            //text!("ingred: {}", self.tList.ingredPos1[n].1.name; x = 0, y = yPos);
        }

        if self.tutorial == 0 && self.day == 0 {
            sprite!("tutorial1", x = 0, y = 0);
        } else if self.tutorial == 1 {
            sprite!("tutorial2", x = 0, y = 0);
        }

        if self.endScreen {
            sprite!("scorescreen", x = 0, y = 0);
            text!("Customer Served!", font = "TENPIXELS", x = 180, y = 130);
            text!("Score: {}", self.totalScore; font = "TENPIXELS", x = 220, y = 150);
            text!("Time: {}", self.timer; font = "TENPIXELS", x = 220, y = 165);       
        }

        //check to see if day continue button is pressed or not
        for n in 0..self.uibuttons.len() {
            let dayPress = self.uibuttons[n].check(select);
            if self.tutorial == 0 && n == 0 {
                self.uibuttons[n].action = false;
            } else if self.tutorial == 1 && n == 4 {
                self.uibuttons[n].action = false;
            } else if !self.endScreen && n == 0 && self.tutorial >= 2{
                self.uibuttons[n].action = false;
            } else if !self.endScreen && n == 5 && self.tutorial <= 3{
                self.uibuttons[n].action = false;
            }
                    //if pressed, goes to next day, resets all track positions, empties soup, and sets soup limit
                    //resetting will all occur here when going to next day for now
                    //eventually will have file reader to load in new ingredient lists, customer orders, etc.
            if self.uibuttons[n].action {

                match n {
                    0 => {
                        self.day += 1;
                        self.tutorial += 1;
                        self.reader.customersDay(self.day);
                        self.uibuttons[0].action = false;
                        self.trackPrint = 0;
                        self.currCus = 0;
                        self.cusCheck = false;
                        self.timer = 0;
                        self.totalScore = 0;
                        self.endScreen = false;
                        self.startDay = true;
                        self.soup.limit = self.reader.customers[0].order.len();
                        self.soup.soup = Vec::new();
                        self.tList.dayIngredients(self.reader.ingredList.clone());
                        for n in 0..8 {
                            self.tList.trackPos1[n] = (0.0,206.0,false);
                            self.tList.trackPos2[n] = (510.0,44.0,false);
                            self.tList.ingredPos1[n].0.hitbox.0 = 0.0;
                            self.tList.ingredPos1[n].0.hitbox.1 = 206.0;
                            self.tList.ingredPos2[n].0.hitbox.0 = 510.0;
                            self.tList.ingredPos2[n].0.hitbox.1 = 44.0;

                            self.tList.ingredPos1[n].1 = Ingredient::new( "empty");
                            self.tList.ingredPos2[n].1 = Ingredient::new("empty");
                        }
                        timer_anim.restart();
                        
                        //log!("{}", self.reader.customers[self.currCus].cusName )
                    }
                    1 => {continue;}
                    2 => {
                        self.soup.dumpSoup();
                        self.uibuttons[2].action = false;
                    }
                    3 => {
                        self.cameraPos.0 = 255;
                        self.uibuttons[3].action = false;
                    }
                    4 => {
                        self.tutorial += 1;
                        self.uibuttons[4].action = false;
                    }
                    5 => {
                        
                        if self.timer != 60 && self.soup.soup.len() > 0{
                            audio::play("bell");
                            audio::set_volume("bell", 0.1);
                            self.reader.customers[self.currCus].serveSoup(self.soup.soup.as_ref());
                            self.currCus += 1;
                            self.timer = 0;
                            self.soup.soup = Vec::new();
                            self.cusTimer = 0;
                        }
                        
                        if self.currCus != self.reader.custNum {
                            self.soup.limit = self.reader.customers[self.currCus].order.len();
                        } else {
                            
                            for n in 0..self.reader.customers.len() {
                                self.totalScore += self.reader.customers[n].calculateScore(self.cusTimer);
                            }
                            self.endScreen = true;
                            self.cusCheck = true;
                        }
                        self.uibuttons[5].action = false;
                    }
                    _ => {}
                
                }   
            }
            if n == 1{
                continue;
            } else if n == 0 && self.tutorial == 1 || self.endScreen && n == 0{
                self.uibuttons[n].draw();
            } else if n == 3 || n == 4 && self.tutorial <= 0{
                self.uibuttons[n].draw();
            } else if n == 5 && self.tutorial >=2 && !self.endScreen{
                self.uibuttons[n].tempDraw(&self.uibuttons[n].text.as_str());
            }
            
        }

        let t = time::tick();    
        log!("{}", self.cusTimer);

        
        
        

        if self.day > 0 && !self.cusCheck{
            //text!("Customer: {}", self.reader.customers[self.currCus].cusName; font = "TENPIXELS", x = 0, y = 270);
            //text!("Order: {:?}", self.reader.customers[self.currCus].order[0].ingredType; font = "TENPIXELS", x = 30, y = 140);
            //text!("{:?}", self.reader.customers[self.currCus].order[1].ingredType; font = "TENPIXELS", x = 30, y = 150);
            //text!("Time Left: {}", 60 - self.timer; font = "TENPIXELS", x = 30, y = 120);

            text!("Ingredients:", x = 25, y = 98, font = "TENPIXELS", color = 0x2d1e1eff);
            self.reader.customers[self.currCus].createOrder(self.cusTimer);
        }
        let mut offsetdashes = 98;
        for n in 0..self.soup.limit {
            offsetdashes += 14;
            text!("-", x = 25, y = offsetdashes, font = "TENPIXELS", color = 0x2d1e1eff);
        }
        let mut offset = 98;
        for n in 0..self.soup.soup.len() {     
            offset += 14;       
            text!("-{}", self.soup.soup[n].name; x = 28, y = offset, font = "TENPIXELS", color = 0x2d1e1eff);            
        }
        

    }


}