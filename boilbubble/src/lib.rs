mod customer;
mod ingredients;
mod track;
mod UIButtons;

use turbo::*;
use turbo::time::tick;
use track::Track;
use crate::UIButtons::UIButtons::UIButton;


#[turbo::game]
struct GameState {
    day: i32,
    timestamp: usize,
    timepass: usize,
    trackList: Track,
    trackPrint: usize,
    nextDay: UIButton,
    uibuttons: [UIButton; 2],
    
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
            nextDay: UIButton::new("NextDay", (115.0, 115.0, 40.0, 20.0), false),
            uibuttons: [
                UIButton::new("NextDay", (106.0, 115.0, 40.0, 20.0), false),
                UIButton::new("soup", (115.0, 70.0, 20.0, 20.0), false),
            ],
        }
    }
    pub fn update(&mut self) {
        let mut select: (f32,f32) = (0.0,0.0);
        let mut select2: (f32,f32) = (0.0,0.0);
        let m = pointer::world();
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
            if self.trackList.ingredPos1[n].1.name == "Sugar" && !self.trackList.trackPos1[n].2{
                self.trackList.ingredPos1[n].0.tempDraw();
            }
            if self.trackList.ingredPos2[n].1.name == "Sugar" && !self.trackList.trackPos2[n].2{
                self.trackList.ingredPos2[n].0.tempDraw();
            }
            //if the track item reaches the end of the screen, then reset it to start
            if !self.trackList.trackPos1[n].2 {
                circ!(x = self.trackList.trackPos1[n].0, y = self.trackList.trackPos1[n].1, d=8, color = 0x32CD32ff);
            } else if self.trackList.trackPos1[n].2 {
                self.trackList.trackPos1[n].2 = false;
                self.trackList.trackPos1[n].0 = 0.0;
            }
            if !self.trackList.trackPos2[n].2 {
                circ!(x = self.trackList.trackPos2[n].0, y = self.trackList.trackPos2[n].1, d=8, color = 0x32CD32ff);
            } else if self.trackList.trackPos2[n].2 {
                self.trackList.trackPos2[n].2 = false;
                self.trackList.trackPos2[n].0 = 250.0;
            }
        }

        //check to see if day continue button is pressed or not
        for n in 0..self.uibuttons.len() {
            let dayPress = self.uibuttons[n].check(select);
                    //if pressed, goes to next day, resets all track positions
            if self.uibuttons[n].action && self.uibuttons[n].text == "NextDay" {
                self.day += 1;
                self.uibuttons[0].action = false;
                self.trackPrint = 0;
                for n in 0..8 {
                    self.trackList.trackPos1[n] = (0.0,100.0,false);
                    self.trackList.trackPos2[n] = (250.0,30.0,false);
                    self.trackList.ingredPos1[n].0.hitbox.0 = 0.0;
                    self.trackList.ingredPos1[n].0.hitbox.1 = 100.0;
                    self.trackList.ingredPos2[n].0.hitbox.0 = 250.0;
                    self.trackList.ingredPos2[n].0.hitbox.1 = 30.0;
                }   
            }
            self.uibuttons[n].tempDraw();
        }

        text!("Day: {}", self.day; x = 0, y = 10);

    }


}