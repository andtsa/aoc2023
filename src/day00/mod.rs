use crate::PART;

// Solution day 00
pub fn solve(input : String, part : PART) -> String {
    match part {
        PART::ONE => {part_one(input)}
        PART::TWO => {part_two(input)}
    }
}
pub fn part_one(input : String) -> String {
    let mut result : String = String::new();

    let mut max = 0;
    let p1 = input.split("\n\n").collect::<Vec<&str>>();
    let mut elves : Vec<i32> = vec![];
    for s in p1 {
        elves.push(
            s.split("\n").collect::<Vec<&str>>()
                .into_iter()
                .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
                .iter()
                .sum()
        );
    }
    elves.sort();
    result.push_str(elves[0..1].iter().sum::<i32>().to_string().as_str());
    return result;
}

pub fn part_two(input : String) -> String {
    let mut result : String = String::new();

    let mut max = 0;
    let p1 = input.split("\n\n").collect::<Vec<&str>>();
    let mut elves : Vec<i32> = vec![];
    for s in p1 {
        elves.push(
            s.split("\n").collect::<Vec<&str>>()
            .into_iter()
                .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
                .iter()
                .sum()
        );
    }
    elves.sort();
    result.push_str(elves[0..3].iter().sum::<i32>().to_string().as_str());
    return result;
}