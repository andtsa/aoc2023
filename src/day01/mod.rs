use crate::PART;
use regex::Regex;


// Solution day 01
pub fn solve(input : String, part : PART) -> String {match part { PART::ONE => {part_one(input)}PART::TWO => {part_two(input)}}}
pub fn part_one(input : String) -> String {
    let re = Regex::new(r"\d").unwrap();
    let mut r = vec![];
    for line in input.lines() {
        let mut results : Vec<String> = vec![];
        for cap in re.captures_iter(&line) {
            results.push(cap[0].to_owned());
        }
        r.push(format!("{}{}", results[0], results[results.len() - 1]));
    }
    // println!("{:?}", r);
    return r.iter().map(|x| x.parse::<i32>().unwrap()).sum::<i32>().to_string();
}

pub fn part_two(input : String) -> String {

    let re = Regex::new(r"\d").unwrap();
    let mut r = vec![];
    for line in input.lines() {
        let mut l = line.replace("one", "o1e");
        l = l.replace("two", "t2e");
        l = l.replace("three", "t3e");
        l = l.replace("four", "f4r");
        l = l.replace("five", "f5e");
        l = l.replace("six", "s6x");
        l = l.replace("seven", "s7n");
        l = l.replace("eight", "e8t");
        l = l.replace("nine", "n9e");
        let mut results : Vec<String> = vec![];
        for cap in re.captures_iter(&l) {
            results.push(cap[0].to_owned());
        }
        r.push(format!("{}{}", results[0], results[results.len() - 1]));
    }
    // println!("{:?}", r);
    return r.iter().map(|x| x.parse::<i32>().unwrap()).sum::<i32>().to_string();
}