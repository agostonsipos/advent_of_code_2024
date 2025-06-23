use std::io::{self, BufRead};
use ndarray::Array2;
use std::collections::{HashMap, HashSet};

// A very ugly solution because I did not give up on actually counting the sides of the polygons,
// all the edge cases made it horrible code though

fn preprocess() -> Array2<char> {
    let stdin = io::stdin();
    let input_content = stdin.lock().lines();
    let mut vec: Vec<char> = vec![];
    let mut n = 0;
    let mut m = 0;
    for line in input_content {
        let line_content = line.unwrap();
        for ch in line_content.chars() {
            vec.push(ch);
        }
        m = line_content.len();
        n += 1;
    }
    return Array2::from_shape_vec((n, m), vec).unwrap();
}

fn recursive_paint(mat: &Array2<char>, visited: &mut HashSet<(usize, usize)>, start: (usize, usize), invert: bool, val: char) -> (i64, i64) {
    if visited.contains(&start) {
        return (0, 0);
    }
    let mut perim = 0;
    let mut area = 1;
    let x = start.0 as i64;
    let y = start.1 as i64;
    visited.insert(start);
    for (a, b) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let ind = ((x + a) as usize, (y + b) as usize);
        if invert && (x + a < 0 || y + b < 0 || mat.get(ind).is_none()) {
            return (-1, -1);
        }
        if x + a < 0 || y + b < 0 || mat.get(ind).is_none() || if invert { *mat.get(ind).unwrap() == val } else { *mat.get(ind).unwrap() != val } {
            perim += 1;
        }
        else {
            let (na, np) = recursive_paint(mat, visited, ind, invert, val);
            if na == -1 {
                return (-1, -1);
            }
            area += na;
            perim += np;
        }
    }
    return (area, perim);
}

fn part1(mat: &Array2<char>) -> i64 {
    let mut area_and_perimeter = HashMap::<(char, (usize, usize)), (i64, i64)>::new();
    let mut visited = HashSet::<(usize, usize)>::new();
    for (ind, val) in mat.indexed_iter() {
        if visited.contains(&ind) { continue; }
        let (a, p) = recursive_paint(mat, &mut visited, ind, false, mat[ind]);
        area_and_perimeter.insert((*val, ind), (a, p));
    }
    let mut x = 0;
    for (_c, (a, p)) in area_and_perimeter {
        x += a * p;
    }
    return x;
}

fn turn_left(dir: (i32, i32)) -> (i32, i32) {
    return (-dir.1, dir.0);
}

fn turn_right(dir: (i32, i32)) -> (i32, i32) {
    return (dir.1, -dir.0);
}

fn add_coord(pos: (usize, usize), dir: (i32, i32)) -> (usize, usize) {
    return ((pos.0 as i32 + dir.0) as usize, (pos.1 as i32 + dir.1) as usize);
}

fn traverse_perimeter_and_count_sides(fields: &HashSet<(usize, usize)>, start: (usize, usize)) -> i64 {
    let mut dir = (0, 1);
    let mut pos = start;
    let mut sides = 0;
    while sides < 2 || pos != start || dir != (0, 1) {
        if fields.contains(&add_coord(pos, turn_left(dir))) {
            dir = turn_left(dir);
            sides += 1;
            pos = add_coord(pos, dir);
        }
        else if fields.contains(&add_coord(pos, dir)) {
            pos = add_coord(pos, dir);
        }
        else {
            dir = turn_right(dir);
            sides += 1;
        }
    }
    return sides;
}

fn is_closed_hole(mat: &Array2<char>, visited: &mut HashSet<(usize, usize)>, pos: (usize, usize), val: char) -> bool {
    if visited.contains(&pos) {
        return true;
    }
    if pos.0 == 0 || pos.1 == 0 || mat.get((pos.0+1, pos.1+1)).is_none() {
        return false;
    }
    visited.insert(pos);
    for x in [-1, 0, 1] {
        for y in [-1, 0, 1] {
            if x == 0 && y == 0 { continue; }
            let newpos = (pos.0+(x as usize), pos.1+(y as usize));
            if mat[newpos] != val {
                if !is_closed_hole(mat, visited, newpos, val) {
                    return false;
                }
            }
        }
    }
    return true;
}

fn part2(mat: &Array2<char>) -> i64 {
    let mut area_and_perimeter = HashMap::<(char, (usize, usize)), (i64, i64)>::new();
    let mut visited = HashSet::<(usize, usize)>::new();
    for (ind, val) in mat.indexed_iter() {
        if visited.contains(&ind) { continue; }
        let (a, p) = recursive_paint(mat, &mut visited, ind, false, mat[ind]);
        area_and_perimeter.insert((*val, ind), (a, p));
    }
    let mut n = 0;
    for ((c, (x, y)), (a, _p)) in area_and_perimeter {
        let mut fields = HashSet::<(usize, usize)>::new();
        recursive_paint(mat, &mut fields, (x, y), false, mat[(x, y)]);
        let mut sides = traverse_perimeter_and_count_sides(&fields, (x, y));
        // count inner holes
        let mut vector = fields.into_iter().collect::<Vec<(usize, usize)>>();
        vector.sort();
        let mut visited2 = HashSet::<(usize, usize)>::new();
        for i in 1..vector.len() {
            if vector[i-1].0 == vector[i].0 {
                let u = vector[i-1].1;
                let v = vector[i].1;
                for j in u+1..v {
                    if visited2.contains(&(vector[i].0, j)) {
                        continue;
                    }
                    let mut fields = HashSet::<(usize, usize)>::new();
                    let mut visited3 = HashSet::<(usize, usize)>::new();
                    if !is_closed_hole(mat, &mut visited3, (vector[i].0, j), c) {
                        continue;
                    }
                    let ret = recursive_paint(mat, &mut fields, (vector[i].0, j), true, mat[vector[i]]);
                    if ret == (-1, -1) {
                        continue;
                    }
                    sides += traverse_perimeter_and_count_sides(&fields, (vector[i].0, j));
                    visited2 = visited2.union(&fields).cloned().collect();
                }
            }
        }
        n += a * sides;
    }
    return n;
}

fn main() {
    let mat = preprocess();
    let x = part1(&mat);
    println!("{}", x);
    let y = part2(&mat);
    println!("{}", y);
}
