pub mod euler;

use std::env;
use euler::get_method;

fn main() {
    let args: Vec<String> = env::args().collect();

    let problem_number = &args[1];
    let answer = run_method(problem_number.as_str());

    println!("{:?}", answer);
}

fn run_method(problem_number_str: &str) -> String {
    match problem_number_str.parse() {
        Ok(problem_number) => get_method(problem_number)(),
        Err(error) => format!("Invalid number '{}'. Error '{}'", problem_number_str, error),
    }
}
