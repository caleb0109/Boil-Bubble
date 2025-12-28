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
                self.current_line = n.unwrap() + 1;
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
                order.push(Ingredient::new(&ingredName));
            }
            
            let cusOrder = Customer::new(name.as_str(), orderDesc.as_str(), order);
            self.customers.push(cusOrder);
        }

        for x in 0..self.ingredNum {
            let ingred = Ingredient::new(self.lines[self.current_line].as_str());
            self.current_line += 1;
            self.ingredList.push(ingred);
        }
    }
}