use std::io::{self, BufRead};

fn preprocess() -> Vec<Vec<i32>> {
    let stdin = io::stdin();
    let input_content = stdin.lock().lines();
    let mut mat: Vec<Vec<i32>> = vec![];
    for line in input_content {
        let mut row: Vec<i32> = vec![];
        let line_content = line.unwrap();
        let numbers: Vec<&str> = line_content.split_whitespace().collect();
        for n in numbers {
            row.push(n.parse::<i32>().unwrap());
        }
        mat.push(row);
    }
    return mat;
}

fn safe(row: &[i32], tolerance: i32) -> bool {
    let mut offending: Option<usize> = None;
    for i in 1..row.len() {
        if (row[i] - row[i-1]).abs() > 3 || (row[i] - row[i-1]) * (row[1] - row[0]) <= 0 {
            offending = Some(i-1);
            break;
        }
    }
    if offending.is_none() {
        return true;
    } else {
        if tolerance == 0 {
            // we can't remove any more elements
            return false;
        }
        // try omitting the first element, maybe it has different monotonity
        if safe(&row[1..], tolerance - 1) {
            return true;
        }
        let index_to_remove = offending.unwrap();
        // try removing first element from the offending diff
        let mut vec1 = row.to_vec();
        vec1.remove(index_to_remove);
        if safe(&vec1, tolerance - 1) {
            return true;
        }
        // try removing second element from the offending diff
        let mut vec2 = row.to_vec();
        vec2.remove(index_to_remove + 1);
        if safe(&vec2, tolerance - 1) {
            return true;
        }
        return false;
    }
}

fn partn(mat: &Vec<Vec<i32>>, tolerance: i32) -> i32 {
    let mut c = 0;
    for row in mat {
        if safe(row, tolerance) {
            c += 1;
        }
    }
    return c;
}

fn part1(mat: &Vec<Vec<i32>>) -> i32 {
    return partn(mat, 0);
}

fn part2(mat: &Vec<Vec<i32>>) -> i32 {
    return partn(mat, 1);
}

fn main() {
    let mat = preprocess();
    let x = part1(&mat);
    println!("{}", x);
    let y = part2(&mat);
    println!("{}", y);
}

