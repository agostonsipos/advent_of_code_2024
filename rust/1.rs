use std::io::{self, BufRead};

fn preprocess() -> (Vec<i32>, Vec<i32>) {
    let stdin = io::stdin();
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for line in stdin.lock().lines() {
        let line_content = line.unwrap();
        let xy: Vec<&str> = line_content.split_whitespace().collect();
        left.push(xy[0].parse::<i32>().unwrap());
        right.push(xy[1].parse::<i32>().unwrap());
    }
    left.sort();
    right.sort();
    return (left, right);
}

fn part1(left: &[i32], right: &[i32]) {
    let mut s = 0;
    for i in 0..left.len() {
    	s += (left[i] - right[i]).abs();
    }
    println!("{}", s);
}

fn count_same(value: i32, sorted_slice: &[i32]) -> i32 {
    let mut c = 0;
    for x in sorted_slice {
        if *x != value {
            return c;
        }
        c += 1;
    }
    return sorted_slice.len() as i32;
}

fn part2(left: &[i32], right: &[i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut s = 0;
    while i < left.len() && j < right.len() {
        if left[i] == right[j] {
            s += left[i] * count_same(left[i], &right[j..]);
            i += 1;
        } else if left[i] < right[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    println!("{}", s);
}

fn main() {
    let (left, right) = preprocess();
    part1(&left, &right);
    part2(&left, &right);
}
