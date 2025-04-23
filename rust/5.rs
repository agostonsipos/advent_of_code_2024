use std::io::{self, BufRead};

fn preprocess() -> (Vec<(i32,i32)>,Vec<Vec<i32>>) {
    let stdin = io::stdin();
    let mut rules: Vec<(i32,i32)> = vec![];
    for line in stdin.lock().lines() {
        let line_content = line.unwrap();
        if line_content == "" {
            break;
        }
        let xy: Vec<&str> = line_content.split("|").collect();
        rules.push((xy[0].parse::<i32>().unwrap(), xy[1].parse::<i32>().unwrap()));
    }
    let mut pages: Vec<Vec<i32>> = vec![];
    for line in stdin.lock().lines() {
        let line_content = line.unwrap();
        let nums: Vec<&str> = line_content.split(",").collect();
        pages.push(nums.iter().map(|s| s.parse::<i32>().unwrap()).collect());
    }
    return (rules, pages);
}

fn parts(rules: &Vec<(i32,i32)>, pages: &mut Vec<Vec<i32>>) -> (i32, i32) {
    let mut correct = 0;
    let mut incorrect = 0;
    for pagelist in pages {
        let mut good = true;
        for i in 0..pagelist.len() {
            for j in i+1..pagelist.len() {
                for rule in rules {
                    if rule.0 == pagelist[j] && rule.1 == pagelist[i] {
                        pagelist.swap(i, j);
                        good = false;
                    }
                }
            }
        }
        if good {
            correct += pagelist[pagelist.len() / 2];
        } else {
            incorrect += pagelist[pagelist.len() / 2];
        }
    }
    return (correct, incorrect);
}

fn main() {
    let (rules, mut pages) = preprocess();
    let (x, y) = parts(&rules, &mut pages);
    println!("{}", x);
    println!("{}", y);
}
