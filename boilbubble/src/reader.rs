use turbo::{text::Text, *};
static SCRIPT_PATH: &str = std::include_str!("script");
use crate::customer::customer::Customer;
use crate::ingredients::ingredients::Ingredient;

#[turbo::serialize]

pub struct Reader {
    pub lines: Vec<String>,
    pub current_line: usize,
    pub custNum: usize,
    pub ingredNum: usize,
    pub customers: Vec<Customer>,
    pub ingredList: Vec<Ingredient>,
}

//file reader in order is:
//1) Amount of customers
//2) Amount of ingredients used that day
//3) Starts customer data, name
//4) order description
//5) amount of ingredients that should be in the order
//6) ingredient names for the order
//7) if multiple customers, repeat steps 3-6 for each customer
//8) ingredient name for the ingredient list used for day
impl Reader {
    pub fn new() -> Self {
        Self {
            lines: SCRIPT_PATH.split("\r\n").map(|line| line.to_string()).collect(),
            current_line: 0,
            custNum: 0,
            ingredNum: 0,
            customers: Vec::new(),
            ingredList: Vec::new(),
        }
    }

    pub fn customersDay(&mut self, day: i32) {
        match day {
            1 => {
                let n = self.lines.iter().position(|line| line == "day1");
                self.current_line = n.unwrap_or(0) + 1;
                self.updateData();
            }
            2 => {
                let n = self.lines.iter().position(|line| line == "day2");
                self.current_line = n.unwrap_or(0) + 1;
                self.updateData();
            }
            3 => {
                let n = self.lines.iter().position(|line| line == "day3");
                self.current_line = n.unwrap_or(0) + 1;
                self.updateData();
            }
            4 => {
                let n = self.lines.iter().position(|line| line == "day4");
                self.current_line = n.unwrap_or(0) + 1;
                self.updateData();
            }
            5 => {
                let n = self.lines.iter().position(|line| line == "day5");
                self.current_line = n.unwrap_or(0) + 1;
                self.updateData();
            }
            6 => {
                let n = self.lines.iter().position(|line| line == "day6");
                self.current_line = n.unwrap_or(0) + 1;
                self.updateData();
            }
            7 => {
                let n = self.lines.iter().position(|line| line == "day7");
                self.current_line = n.unwrap_or(0) + 1;
                self.updateData();
            }
            8 => {
                let n = self.lines.iter().position(|line| line == "day8");
                self.current_line = n.unwrap_or(0) + 1;
                self.updateData();
            }
            9 => {
                let n = self.lines.iter().position(|line| line == "day9");
                self.current_line = n.unwrap_or(0) + 1;
                self.updateData();
            }
            10 => {
                let n = self.lines.iter().position(|line| line == "day10");
                self.current_line = n.unwrap_or(0) + 1;
                self.updateData();
            }
            _ => {}
        }
    }

    pub fn updateData(&mut self) {
        self.custNum = self.lines[self.current_line].parse::<usize>().unwrap();
        self.current_line += 1;
        self.ingredNum = self.lines[self.current_line].parse::<usize>().unwrap();
        self.current_line += 1;
        
        for x in 0..self.custNum {
            let name = self.lines[self.current_line].clone();
            self.current_line += 1;
            let orderDesc = self.lines[self.current_line].clone();
            self.current_line += 1;

            let orderSize = self.lines[self.current_line].parse::<usize>().unwrap();
            self.current_line += 1;


            let mut order: Vec<Ingredient> = Vec::new();

            for y in 0..orderSize {
                let ingredName= self.lines[self.current_line].clone();
                self.current_line += 1;
                order.push(Ingredient::new("empty"));
                order[y].setType(&ingredName);
            }
            
            let cusOrder = Customer::new(name.as_str(), orderDesc.as_str(), order);
            self.customers.push(cusOrder);

        }

        for x in 0..self.ingredNum {
            let ingred = Ingredient::new(self.lines[self.current_line].as_str());
            self.ingredList.push(ingred);
            self.ingredList[x].setType(&self.lines[self.current_line]);
            self.current_line += 1;
        }
    }

    pub fn reset(&mut self) {
        self.current_line = 0;
        self.custNum = 0;
        self.ingredNum = 0;
        self.customers = Vec::new();
        self.ingredList = Vec::new();
    }
}