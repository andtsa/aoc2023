mod day01;
mod day00;
use std::time::Instant;

const DAY: &str = "01";

fn run(input : String, part: PART) -> String {
    return day01::solve(input, part);
}
fn main() {
    println!("Starting Advent of Code 2023 day {}...", DAY);

    println!("Part One:");
    let mut start = Instant::now();
    let mut out = run(read_test(&DAY, PART::ONE), PART::ONE);
    println!("
Finished test in {}ms
output= ` {} `
expected = ` {} `", start.elapsed().as_millis(), out, get_expected(&DAY, PART::ONE));

    start = Instant::now();
    out = run(read_input(&DAY), PART::ONE);
    println!("
Finished part one in {}ms
output = ` {} ` ", start.elapsed().as_millis(), out);

    println!("Part Two:");
    let mut start = Instant::now();
    let mut out = run(read_test(&DAY, PART::TWO), PART::TWO);
    println!("
Finished test in {}ms
output= ` {} `
expected = ` {} `", start.elapsed().as_millis(), out, get_expected(&DAY, PART::TWO));

    start = Instant::now();
    out = run(read_input(&DAY), PART::TWO);
    println!("
Finished part two in {}ms
output = ` {} ` ", start.elapsed().as_millis(), out);
}

pub fn read_test(d : &str, part : PART) -> String {
    return match part {
        PART::ONE => { std::fs::read_to_string(format!("/Users/andtsa/Documents/adventofcode/src/day{}/test-one", d)).unwrap() }
        PART::TWO => { std::fs::read_to_string(format!("/Users/andtsa/Documents/adventofcode/src/day{}/test-two", d)).unwrap() }
    }
}
pub fn read_input(d : &str) -> String {
    return std::fs::read_to_string(format!("/Users/andtsa/Documents/adventofcode/src/day{}/input", d)).unwrap();
}
pub fn get_expected(d : &str, part : PART) -> String {
    return match part {
        PART::ONE => { std::fs::read_to_string(format!("/Users/andtsa/Documents/adventofcode/src/day{}/answer-one", d)).unwrap() }
        PART::TWO => { std::fs::read_to_string(format!("/Users/andtsa/Documents/adventofcode/src/day{}/answer-two", d)).unwrap() }
    }
}

pub enum PART {
    ONE,
    TWO
}
