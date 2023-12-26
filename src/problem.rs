use rand::seq::SliceRandom;
use rand::Rng;

pub struct Problem {
    pub num1: i32,
    pub num2: i32,
    pub operator: char,
}

impl Problem {
    pub fn new() -> Problem {
        Problem {
            num1: 0,
            num2: 0,
            operator: '+',
        }
    }

    pub fn create(&mut self) {
        let mut rng = rand::thread_rng();
        let options = vec!['+', '-', '*', '/'];
        let choice = options.choose(&mut rng);
        let range = 1..101;

        self.num1 = rng.gen_range(range.clone());
        self.num2 = rng.gen_range(range);
        self.operator = *choice.unwrap();
    }

    pub fn check(&self) -> i32 {
        let mut answer = 0;
        match self.operator {
            '+' => answer = self.num1 + self.num2,
            '-' => answer = self.num1 - self.num2,
            '*' => answer = self.num1 * self.num2,
            '/' => answer = self.num1 / self.num2,
            _ => println!("演算子が不正です。"),
        }

        return answer;
    }
}
