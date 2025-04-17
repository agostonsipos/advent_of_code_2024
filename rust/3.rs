use std::io::{self, Read};

use regex::Regex;

enum Statement {
    Do,
    Dont,
    Mul((i32,i32)),
}

fn preprocess() -> Vec<Statement> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let _res = stdin.lock().read_to_string(&mut buffer);
    let pattern = r"(do\(\))|(don't\(\))|mul\((\d+)\,(\d+)\)";
    let re = Regex::new(pattern).unwrap();
    let mut data: Vec<Statement> = vec![];
    for cap in re.captures_iter(&buffer) {
        let match_as_int = |m: Option<regex::Match>| m.unwrap().as_str().parse::<i32>().unwrap();
        if cap.get(3).is_some() {
            data.push(Statement::Mul((match_as_int(cap.get(3)), match_as_int(cap.get(4)))));
        } else if cap.get(1).is_some() {
            data.push(Statement::Do);
        } else if cap.get(2).is_some() {
            data.push(Statement::Dont);
        }
    }
    return data;
}

fn part1(data: &Vec<Statement>) -> i32 {
    let mut s = 0;
    for m in data {
        match m {
            Statement::Mul((x, y)) => s += x * y,
            Statement::Do => (),
            Statement::Dont => (),
        }
    }
    return s;
}

fn part2(data: &Vec<Statement>) -> i32 {
    let mut s = 0;
    let mut f = 1;
    for m in data {
        match m {
            Statement::Mul((x, y)) => s += f * x * y,
            Statement::Do => f = 1,
            Statement::Dont => f = 0,
        }
    }
    return s;
}

fn main() {
    let data = preprocess();
    let x = part1(&data);
    println!("{}", x);
    let y = part2(&data);
    println!("{}", y);
}
