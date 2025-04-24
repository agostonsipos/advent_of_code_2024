use std::io::{self, BufRead};

fn preprocess() -> Vec<(i64, Vec<i64>)> {
    let stdin = io::stdin();
    let input_content = stdin.lock().lines();
    let mut inp: Vec<(i64, Vec<i64>)> = vec![];
    for line in input_content {
        let mut row: Vec<i64> = vec![];
        let line_content = line.unwrap();
        let pair: Vec<&str> = line_content.split(": ").collect();
        let lhs = pair[0].parse::<i64>().unwrap();
        for x in pair[1].split_whitespace() {
            row.push(x.parse::<i64>().unwrap());
        }
        inp.push((lhs, row));
    }
    return inp;
}

fn is_solvable(lhs: i64, rhs: &[i64]) -> bool {
    if lhs == 0 && rhs.len() == 0 {
        return true;
    } else if rhs.len() == 0 {
        return false;
    }
    let last = rhs[rhs.len()-1];
    let mut ok = is_solvable(lhs - last, &rhs[0..rhs.len()-1]);
    if lhs % last == 0 {
        ok = ok || is_solvable(lhs / last, &rhs[0..rhs.len()-1]);
    }
    return ok;
}

fn chop_postfix(lhs: i64, rhs: i64) -> Option<i64> {
    let s1 = lhs.to_string();
    let s2 = rhs.to_string();
    if s1.len() > s2.len() && s1[s1.len()-s2.len()..s1.len()] == s2 {
        return Some(s1[0..s1.len()-s2.len()].parse::<i64>().unwrap());
    }
    return None;
}

fn is_solvable_with_concat(lhs: i64, rhs: &[i64]) -> bool {
    if lhs == 0 && rhs.len() == 0 {
        return true;
    } else if lhs < 0 || rhs.len() == 0 {
        return false;
    }
    let last = rhs[rhs.len()-1];
    let mut ok = is_solvable_with_concat(lhs - last, &rhs[0..rhs.len()-1]);
    if lhs % last == 0 {
        ok = ok || is_solvable_with_concat(lhs / last, &rhs[0..rhs.len()-1]);
    }
    let chopped = chop_postfix(lhs, last);
    if chopped.is_some() {
        ok = ok || is_solvable_with_concat(chopped.unwrap(), &rhs[0..rhs.len()-1]);
    }
    return ok;
}

fn partn(inp: &Vec<(i64, Vec<i64>)>, functor: fn(lhs: i64, rhs: &[i64]) -> bool) -> i64 {
    let mut s = 0;
    for line in inp {
        if functor(line.0, &line.1) {
            s += line.0;
        }
    }
    return s;
}

fn main() {
    let inp = preprocess();
    let x = partn(&inp, is_solvable);
    println!("{}", x);
    let y = partn(&inp, is_solvable_with_concat);
    println!("{}", y);
}