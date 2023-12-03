use crate::PART;
use regex::Regex;

const RED : i32 = 12;
const GREEN : i32 = 13;
const BLUE : i32 = 14;

// Solution day xx
pub fn solve(input : String, part : PART) -> String {
    match part {
        PART::ONE => {part_one(input)}
        PART::TWO => {part_two(input)}
    }
}
pub fn part_one(input : String) -> String {
    let mut valid_games : Vec<i32> = vec![];
    for line in input.split("\n") {
        // println!("{}", line);
        let game_numer = Regex::new("Game (\\d+):").unwrap().captures(line).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        // Regex::new("(\\d+) blue").unwrap().captures(line).unwrap().iter().map(|x| x.unwrap().as_str()).for_each(|x| println!("{}", x));
        let max_blue = Regex::new("(\\d+) blue").unwrap().captures(line).unwrap().iter().map(|x| x.unwrap().as_str().replace(" blue", "").parse::<i32>().unwrap()).max().unwrap();
        let max_green = Regex::new("(\\d+) green").unwrap().captures(line).unwrap().iter().map(|x| x.unwrap().as_str().replace(" green", "").parse::<i32>().unwrap()).max().unwrap();
        let max_red = Regex::new("(\\d+) red").unwrap().captures(line).unwrap().iter().map(|x| x.unwrap().as_str().replace(" red", "").parse::<i32>().unwrap()).max().unwrap();
        if max_blue <= BLUE && max_green <= GREEN && max_red <= RED {
            valid_games.push(game_numer);
        }
        println!("{} {} {} {}", max_blue, max_green, max_red, game_numer);
    }
    println!("{:?}", valid_games);
    return valid_games.iter().sum::<i32>().to_string()
}

pub fn part_two(input : String) -> String {
    let mut game_powers : Vec<i32> = vec![];
    for line in input.split("\n") {
        let max_blue = Regex::new("(\\d+) blue").unwrap().captures(line).unwrap().iter().map(|x| x.unwrap().as_str().replace(" blue", "").parse::<i32>().unwrap()).max().unwrap();
        let max_green = Regex::new("(\\d+) green").unwrap().captures(line).unwrap().iter().map(|x| x.unwrap().as_str().replace(" green", "").parse::<i32>().unwrap()).max().unwrap();
        let max_red = Regex::new("(\\d+) red").unwrap().captures(line).unwrap().iter().map(|x| x.unwrap().as_str().replace(" red", "").parse::<i32>().unwrap()).max().unwrap();
        game_powers.push(max_red*max_green*max_blue);
    }
    return game_powers.iter().sum::<i32>().to_string()
}