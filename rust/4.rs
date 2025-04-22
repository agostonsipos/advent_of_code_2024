use std::io::{self, BufRead};

fn preprocess() -> Vec<Vec<char>> {
    let stdin = io::stdin();
    let input_content = stdin.lock().lines();
    let mut mat: Vec<Vec<char>> = vec![];
    for line in input_content {
        let line_content = line.unwrap();
        let row: Vec<char> = line_content.chars().collect();
        mat.push(row);
    }
    return mat;
}

fn get(mat: &Vec<Vec<char>>, i: i32, j: i32) -> char {
    let n = mat.len() as i32;
    let m = mat[0].len() as i32;
    if i < 0 || i >= n || j < 0 || j >= m {
        return '.';
    }
    return mat[i as usize][j as usize];
}

fn ith_direction(i: usize) -> (i32, i32) {
    let dx: i32 = match i {
        5 | 6 | 7 => 1,
        0 | 1 | 2 => -1,
        _ => 0, // 3 and 4
    };
    let dy: i32 = match i {
        0 | 3 | 5 => 1,
        2 | 4 | 7 => -1,
        _ => 0, // 1 and 6
    };
    return (dx, dy);
}

fn part1(mat: &Vec<Vec<char>>) -> i32 {
    let n = mat.len() as i32;
    let m = mat[0].len() as i32;
    let mut c = 0;
    let pattern: Vec<char> = "XMAS".chars().collect();
    for i in 0..n {
        for j in 0..m {
            if get(mat, i, j) == pattern[0] {
                for d in 0..8 {
                    let mut x = i;
                    let mut y = j;
                    let (dx, dy) = ith_direction(d);
                    let mut good = true;
                    for k in 1..4 {
                        x += dx;
                        y += dy;
                        if get(mat, x, y) != pattern[k] {
                            good = false;
                            break;
                        }
                    }
                    if good {
                        c += 1;
                    }
                }
            }
        }
    }
    return c;
}


fn part2(mat: &Vec<Vec<char>>) -> i32 {
    let n = mat.len() as i32;
    let m = mat[0].len() as i32;
    let mut c = 0;
    for i in 0..n {
        for j in 0..m {
            if get(mat, i, j) == 'A' {
                let good_dir = |d: &usize| -> bool {
                    let (dx, dy) = ith_direction(*d);
                    let c1 = get(mat, i-dx, j-dy);
                    let c2 = get(mat, i+dx, j+dy);
                    return c1 == 'M' && c2 == 'S' || c2 == 'M' && c1 == 'S';
                };
                // 0 and 5 are two orthogonal diagonal directions
                if [0,5].iter().all(good_dir) {
                    c += 1;
                }
            }
        }
    }
    return c;
}

fn main() {
    let mat = preprocess();
    let x = part1(&mat);
    println!("{}", x);
    let y = part2(&mat);
    println!("{}", y);
}
