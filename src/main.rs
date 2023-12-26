mod problem;

use problem::Problem;
use std::io;
use std::io::Write;

fn read_input() -> String {
    io::stdout().flush().unwrap();
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("入力エラー");
    return guess;
}

fn main() {
    let mut problem = Problem::new();
    problem.create();
    let answer = problem.check();
    println!("{} {} {} = ?", problem.num1, problem.operator, problem.num2);
    let guess = read_input();
    println!(
        "{}, {}",
        answer,
        answer == guess.trim().parse::<i32>().unwrap()
    );
}
