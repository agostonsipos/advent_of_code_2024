use std::io::{self, BufRead};
use ndarray::Array2;
use std::collections::HashSet;


fn preprocess() -> Array2<u32> {
    let stdin = io::stdin();
    let input_content = stdin.lock().lines();
    let mut vec: Vec<u32> = vec![];
    let mut n = 0;
    for line in input_content {
        let line_content = line.unwrap();
        for ch in line_content.chars() {
            vec.push(ch.to_digit(10).unwrap());
        }
        n += 1;
    }
    let mat = Array2::from_shape_vec((n, n), vec).unwrap();
    return mat;
}

fn part1(mat: &Array2<u32>) -> u64 {
    let mut paths = Array2::<HashSet<(usize, usize)>>::from_elem(mat.dim(), HashSet::new());
    for (ind, val) in mat.indexed_iter() {
        if *val == 9 {
            paths[ind].insert(ind);
        }
    }
    for i in (0..=8).rev() {
        for (ind, val) in mat.indexed_iter() {
            if *val == i {
                let neighbours = [(if ind.0 == 0 { 0 } else { ind.0 - 1 }, ind.1), (ind.0 + 1, ind.1), (ind.0, if ind.1 == 0 { 0 } else { ind.1 - 1}), (ind.0, ind.1 + 1)];
                for n in neighbours {
                    if mat.get(n).is_some() && *mat.get(n).unwrap() == i + 1 {
                        let paths_from_n = paths[n].clone();
                        paths[ind].extend(paths_from_n);
                    }
                }
            }
        }
    }
    let mut s: u64 = 0;
    for (ind, val) in mat.indexed_iter() {
        if *val == 0 {
            s += paths[ind].len() as u64;
        }
    }
    return s;
}

// same as above but we don't even need the sets and union them, just the numbers and add them
fn part2(mat: &Array2<u32>) -> u64 {
    let mut paths = Array2::<i32>::zeros(mat.dim());
    for (ind, val) in mat.indexed_iter() {
        if *val == 9 {
            paths[ind] = 1
        }
    }
    for i in (0..=8).rev() {
        for (ind, val) in mat.indexed_iter() {
            if *val == i {
                let neighbours = [(if ind.0 == 0 { 0 } else { ind.0 - 1 }, ind.1), (ind.0 + 1, ind.1), (ind.0, if ind.1 == 0 { 0 } else { ind.1 - 1}), (ind.0, ind.1 + 1)];
                for n in neighbours {
                    if mat.get(n).is_some() && *mat.get(n).unwrap() == i + 1 {
                        paths[ind] += paths[n];
                    }
                }
            }
        }
    }
    let mut s: u64 = 0;
    for (ind, val) in mat.indexed_iter() {
        if *val == 0 {
            s += paths[ind] as u64;
        }
    }
    return s;
}


fn main() {
    let mat = preprocess();
    let x = part1(&mat);
    println!("{}", x);
    let y = part2(&mat);
    println!("{}", y);
}

