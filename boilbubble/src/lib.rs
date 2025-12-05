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
        let m = pointer::world();
        //for every 1.5 seconds that pass, the next item on the track will appear
        if time::tick() % 90 == 0 && self.trackPrint <= 9 && self.day > 0{
            self.trackPrint += 1;
        }
        //for loop to create the track
        for n in 0..self.trackPrint {
            select = self.trackList.ingredPos[n].0.check(select);
            //checks if the track item is at the end of the opposite side from start
            if !self.trackList.trackPos[n].3 {
                //if the track item has yet to reach the max height and is on starting side
                if self.trackList.trackPos[n].1 <= 105.0 && !self.trackList.trackPos[n].2{
                    self.trackList.trackPos[n].1 += 0.175;
                    self.trackList.trackPos[n].0 += 0.625;
                }
                //if track item reaches max height
                if self.trackList.trackPos[n].1 >= 105.0 {
                    self.trackList.trackPos[n].2 = true;
                    self.trackList.trackPos[n].1 -= 0.175;
                    self.trackList.trackPos[n].0 += 0.625;
                }
                //if track item reached max height, now ascending and making sure it doesn't go below starting height
                if self.trackList.trackPos[n].1 <= 105.0 && self.trackList.trackPos[n].2 && self.trackList.trackPos[n].1 >= 70.175{
                    self.trackList.trackPos[n].1 -= 0.175;
                    self.trackList.trackPos[n].0 += 0.625;
                }
                //if track item reaches end of sceen on opposite side
                if self.trackList.trackPos[n].0 >= 248.0 {
                    self.trackList.trackPos[n].3 = true;
                    self.trackList.trackPos[n].2 = false;
                    self.trackList.trackPos[n].1 = 70.0;
                
                }
            }
            //checks if the track item is on the way back to starting side
            if self.trackList.trackPos[n].3{
                //if the track item reaches min height and is on starting side
                if self.trackList.trackPos[n].1 >= 35.0 && !self.trackList.trackPos[n].2{
                    self.trackList.trackPos[n].1 -= 0.175;
                    self.trackList.trackPos[n].0 -= 0.625;
                }
                //if track item reaches min height
                if self.trackList.trackPos[n].1 <= 35.0 {
                    self.trackList.trackPos[n].2 = true;
                    self.trackList.trackPos[n].1 += 0.175;
                    self.trackList.trackPos[n].0 -= 0.625;
                }
                //if track item reached min height, now descending and making sure it doesn't go above starting height
                if self.trackList.trackPos[n].1 >= 35.0 && self.trackList.trackPos[n].2 && self.trackList.trackPos[n].1 <= 70.175{
                    self.trackList.trackPos[n].1 += 0.175;
                    self.trackList.trackPos[n].0 -= 0.625;
                }

                //will add code for restarting track so is endless loop of rotation
            
            }
            
            //if the ingredient isn't being held, then set its position to the track position
            if !self.trackList.ingredPos[n].0.action {
                self.trackList.ingredPos[n].0.hitbox.0 = self.trackList.trackPos[n].0;
                self.trackList.ingredPos[n].0.hitbox.1 = self.trackList.trackPos[n].1;
            }
            //if the pointer releases the ingredient, ingredient is not active
            if m.just_released() {
                self.trackList.ingredPos[n].0.action = false;
            }
            //if it has a specific name, then draw rect to see difference
            if self.trackList.ingredPos[n].1.name == "Sugar" {
                self.trackList.ingredPos[n].0.tempDraw();
            }
            //will print track plate always, regardless of ingredient
            circ!(x = self.trackList.trackPos[n].0, y = self.trackList.trackPos[n].1, d=8, color = 0x32CD32ff);
            
        }

        //check to see if day continue button is pressed or not
        for n in 0..self.uibuttons.len() {
            let dayPress = self.uibuttons[n].check(select);
                    //if pressed, goes to next day, resets all track positions
            if self.uibuttons[n].action && self.uibuttons[n].text == "NextDay" {
                self.day += 1;
                self.uibuttons[0].action = false;
                self.trackPrint = 0;
                for n in 0..10 {
                    self.trackList.trackPos[n] = (0.0,70.0,false,false);
                    self.trackList.ingredPos[n].0.hitbox.0 = 0.0;
                    self.trackList.ingredPos[n].0.hitbox.1 = 70.0;
                }   
            }
            self.uibuttons[n].tempDraw();
        }

        text!("Day: {}", self.day; x = 0, y = 10);

    }


}