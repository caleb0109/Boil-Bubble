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
    cusTimer: usize,
    cusLim: usize,
    timerSpeed: f32,
    tutorial: usize,
    timeStamp: usize,
    totalScore: i32,
    startDay: bool,
    checked: Vec<bool>,
    endScreen: bool,
    cusReaction: bool,
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
                UIButton::new("continue", (195.0, 230.0, 100.0, 20.0), false),
                UIButton::new("serve", (26.0, 174.0, 94.0, 18.0), false),
            ],
            soup: Soup::new(),
            reader: reader::Reader::new(),
            currCus: 0,
            cusCheck: false,
            cameraPos: (765,143),
            cusTimer: 0,
            cusLim: 24,
            timerSpeed: 1.0,
            tutorial: 0,
            timeStamp: time::tick(),
            totalScore: 0,
            startDay: false,
            checked: vec![false; 8],
            endScreen: false,
            cusReaction: false,
        }
    }
    pub fn update(&mut self) {

        match self.day {
            5 => {
                //20 seconds
                self.timerSpeed = 1.2;
                self.cusLim = 20;
            }
            8 => {
                //18 seconds
                self.timerSpeed = 1.35;
                self.cusLim = 18;
            }
            11 => {
                //16 seconds
                self.timerSpeed = 1.5;
                self.cusLim = 16;
            }
            _ => {}
        }
        //sprites that cannot be interacted with
        sprite!("titlescreen", x = 510, y = 0);
        sprite!("background", x= 0, y = 0);
        sprite!("cauldron", x = 145, y = 148);
        sprite!("cat", x = 181, y =65);
        sprite!("bowls_lowertrack", x = 0, y = 0);
        sprite!("bowls_uppertrack", x = 0, y = 0);
        
        
        //UI
        
        sprite!("borders", x = 0, y = 0);
        
        //timer
        let timer_anim = animation::get("timer");
        timer_anim.use_sprite("timer");

        if self.day > 0 && !self.cusCheck{
            
            sprite!(animation_key = "timer", x = 444, y = 80);
            timer_anim.set_speed(self.timerSpeed);
            sprite!("customer_speech", x = 59, y = 266);
            sprite!("list", x = 4, y = 88);
        } 

        if !audio::is_playing("boil_and_bubble") {
            audio::play("boil_and_bubble");
            audio::set_volume("boil_and_bubble", 0.1);
        }

        self.uibuttons[1].draw();
       
        

        camera::set_xy(self.cameraPos.0, self.cameraPos.1);

        let mut select: (f32,f32) = (0.0,0.0);
        let mut select2: (f32,f32) = (0.0,0.0);
        let m = pointer::world();
        let(mx, my) = m.xy();
        let x = mx as f32;
        let y = my as f32;

        //for every 5/6 of a second that pass, the next item on the track will appear
        //is not perfect for sure, but visually works for now
        //will look into further ways to optimize
        if time::tick() % 64 == 0 && self.trackPrint <= 7 && self.day > 0{
            self.trackPrint += 1;
        }

        if time::tick() % 60 == 0 && self.day > 0 && !self.cusCheck && self.cusTimer <= self.cusLim{
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
               self.uibuttons[1].hover(self.uibuttons[1].hitbox, x, y) && m.just_released() &&
               self.tList.ingredPos1[n].1.name != "empty" {
                self.soup.addIngredients(self.tList.ingredPos1[n].1.clone());
                audio::play("splash");
                audio::set_volume("splash", 0.1);
                //self.tList.ingredPos2[n].1.ingredType = crate::ingredients::IngredientType::Empty;
                self.tList.ingredPos1[n].1.name = "empty".to_string();
                self.tList.ingredPos1[n].1.setType("empty");
                self.tList.ingredPos1[n].0.action = false;
                self.ingredHold = false;
            } 
            if self.tList.ingredPos2[n].0.hover(self.tList.ingredPos2[n].0.hitbox, x, y) && 
               self.uibuttons[1].hover(self.uibuttons[1].hitbox, x, y) && m.just_released() && 
               self.tList.ingredPos2[n].1.name != "empty"{
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
            } 
            if self.tList.ingredPos2[n].0.action && !self.ingredHold{
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
            
            // BUG IN HERE LOOK AT THIS WHEN YOU CAN

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
                self.ingredCheck = 0;
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
            if !self.tList.trackPos1[n].2 && !self.endScreen{
                sprite!("bowl", x = self.tList.ingredPos1[n].0.hitbox.0, y = self.tList.ingredPos1[n].0.hitbox.1);
                sprite!(&self.tList.ingredPos1[n].1.name, x = self.tList.ingredPos1[n].0.hitbox.0, y = self.tList.ingredPos1[n].0.hitbox.1 - 11.0);
            } else if self.tList.trackPos1[n].2 {
                self.startDay = false;
                self.tList.ingredPos1[n].1 = self.tList.ingredientGen();
                self.tList.trackPos1[n].2 = false;
                self.tList.trackPos1[n].0 = 0.0;
            }
            if !self.tList.trackPos2[n].2 && !self.endScreen{
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

        //prints out ingredient name when hovering over ingredient
        //have to do it in separate loop to avoid the textbox from being printed
        //too early and being overlapped by other ingredients
        for n in 0..self.trackPrint {
            if self.tList.ingredPos1[n].0.hover(self.tList.ingredPos1[n].0.hitbox, x, y) && self.tList.ingredPos1[n].1.name != "empty" {
                rect!(x = self.tList.ingredPos1[n].0.hitbox.0 + 55.0, 
                    y = self.tList.ingredPos1[n].0.hitbox.1 - 10.0, 
                    w = self.tList.ingredPos1[n].1.name.len() as f32 * 6.0 + 26.5,
                    h = 20.0,
                    border_size = 1,
                    border_radius = 4,
                    border_color = 0x000000ff,
                    );
                text!(&self.tList.ingredPos1[n].1.name, x = self.tList.ingredPos1[n].0.hitbox.0 + 58.0, y = self.tList.ingredPos1[n].0.hitbox.1 - 7.0, font = "TENPIXELS", color = 0x000000ff);
            } else if self.tList.ingredPos2[n].0.hover(self.tList.ingredPos2[n].0.hitbox, x, y) && self.tList.ingredPos2[n].1.name != "empty" {
                rect!(x = self.tList.ingredPos2[n].0.hitbox.0 + 55.0, 
                    y = self.tList.ingredPos2[n].0.hitbox.1 - 10.0, 
                    w = self.tList.ingredPos2[n].1.name.len() as f32 * 6.0 + 26.5,
                    h = 20.0,
                    border_size = 1,
                    border_radius = 4,
                    border_color = 0x000000ff,
                    );
                text!(&self.tList.ingredPos2[n].1.name, x = self.tList.ingredPos2[n].0.hitbox.0 + 58.0, y = self.tList.ingredPos2[n].0.hitbox.1 - 7.0, font = "TENPIXELS", color = 0x000000ff);
            }
        }

        if self.tutorial == 0 && self.day == 0 {
            sprite!("tutorial1", x = 0, y = 0);
        } else if self.tutorial == 1 {
            sprite!("tutorial2", x = 0, y = 0);
        }

        if self.endScreen {
            let mut yOffset = 0;
            sprite!("scorescreen", x = 0, y = 0);
            text!("Customer Served!", font = "TENPIXELS", x = 180, y = 90);
            for n in 0..self.reader.customers.len() {
                let scoreText = format!("{}: {}/{} = {} pts", &self.reader.customers[n].cusName, self.reader.customers[n].score, self.reader.customers[n].order.len(), self.reader.customers[n].total);
                yOffset += 15;
                text!(&scoreText, font = "TENPIXELS", x = 180, y = 105 + yOffset);
            }       
            text!("Total Score: {}", self.totalScore; font = "TENPIXELS", x = 180, y = 180);
        }

        //check to see if day continue button is pressed or not
        for n in 0..self.uibuttons.len() {
            if n == 2 {
                continue;
            }
            let dayPress = self.uibuttons[n].check(select);
            if self.tutorial == 0 && n == 0 {
                self.uibuttons[n].action = false;
            } else if self.tutorial == 1 && n == 4 {
                self.uibuttons[n].action = false;
            } else if !self.endScreen && n == 0 && self.tutorial >= 2{
                self.uibuttons[n].action = false;
            } else if !self.endScreen && n == 5 && self.tutorial <= 2{
                self.uibuttons[n].action = false;
            } else if self.endScreen && n != 0 {
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
                        self.reader.reset();
                        log!("{}", self.day);
                        self.reader.customersDay(self.day);
                        log!("hi");
                        self.uibuttons[0].action = false;
                        self.trackPrint = 0;
                        self.currCus = 0;
                        self.cusCheck = false;
                        self.cusTimer = 0;
                        self.totalScore = 0;
                        self.endScreen = false;
                        self.startDay = true;
                        self.checked = vec![false; 8];
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
                        //self.soup.dumpSoup();
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
                        
                        if self.cusTimer != self.cusLim && self.soup.soup.len() > 0{
                            audio::play("bell");
                            audio::set_volume("bell", 0.1);
                            self.reader.customers[self.currCus].serveSoup(self.soup.soup.as_ref());
                            self.totalScore += self.reader.customers[self.currCus].calculateScore(self.cusTimer, self.cusLim);
                            //self.reader.customers[self.currCus].drawScoreReaction();
                            //sprite!("sadcustomer", x = 80, y = 200);
                            self.cusReaction = true;
                            self.currCus += 1;
                            self.cusTimer = 0;
                            self.soup.soup = Vec::new();
                            timer_anim.restart();
                        }
                        
                        if self.currCus != self.reader.custNum {
                            self.soup.limit = self.reader.customers[self.currCus].order.len();
                        } else {
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
            } else if n == 5 && self.tutorial >=2 && !self.endScreen && self.soup.soup.len() == 0 {
                self.uibuttons[n].nonselect();
            } else if n == 5 && self.tutorial >=2 && !self.endScreen{
                self.uibuttons[n].draw();
            }
            
        }
        

        //customer reaction anims
        if self.cusReaction == true {
            self.reader.customers[self.currCus-1].drawScoreReaction();
            if self.reader.customers[self.currCus-1].drawScoreReaction() {
                self.cusReaction = false;
            }
            log!("{}", self.reader.customers[self.currCus-1].drawScoreReaction());
        }

        let t = time::tick();    

        


        
        if self.cusTimer == self.cusLim {
            if self.currCus != self.reader.custNum - 1 {
                // audio::play("bell");
                // audio::set_volume("bell", 0.1);
                //maybe new audio? steps walking away lmao
                self.totalScore += self.reader.customers[self.currCus].calculateScore(self.cusTimer, self.cusLim);
                self.currCus += 1;
                self.soup.soup = Vec::new();
                self.cusTimer = 0;
                timer_anim.restart();
            } else {
                self.endScreen = true;
                self.cusCheck = true;
            }
            
            
        }
        

        if self.day > 0 && !self.cusCheck{
            //text!("Customer: {}", self.reader.customers[self.currCus].cusName; font = "TENPIXELS", x = 0, y = 270);
            //text!("Order: {:?}", self.reader.customers[self.currCus].order[0].ingredType; font = "TENPIXELS", x = 30, y = 140);
            //text!("{:?}", self.reader.customers[self.currCus].order[1].ingredType; font = "TENPIXELS", x = 30, y = 150);
            //text!("Time Left: {}", 60 - self.timer; font = "TENPIXELS", x = 30, y = 120);
            text!("Ingredients:", x = 25, y = 98, font = "TENPIXELS", color = 0x2d1e1eff);
            text!("Day: {}", self.day; x = 10, y = 5, font = "TENPIXELS");
            self.reader.customers[self.currCus].createOrder(self.cusTimer, self.day);
        }
        let mut offsetdashes = 98;
        for n in 0..self.soup.limit {
            offsetdashes += 14;
            text!("-", x = 25, y = offsetdashes, font = "TENPIXELS", color = 0x2d1e1eff);
        }
        let mut offset = 98;
        for n in 0..self.soup.soup.len() {     
            offset += 14;       
            text!("{}", self.soup.soup[n].name; x = 33, y = offset, font = "TENPIXELS", color = 0x2d1e1eff);            
        }
        
        

    }


}