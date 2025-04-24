use std::io::{self, BufRead};
use ndarray::Array2;
use std::ops::Add;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord2D(i32, i32);

impl Add for Coord2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coord2D(self.0 + other.0, self.1 + other.1)
    }
}

impl Coord2D {
    fn new(xy: (usize, usize)) -> Coord2D {
        return Coord2D(xy.0 as i32, xy.1 as i32);
    }
    fn as_index(&self) -> (usize, usize) {
        return (self.0 as usize, self.1 as usize);
    }
}

fn preprocess() -> (Array2<char>, (usize, usize)) {
    let stdin = io::stdin();
    let input_content = stdin.lock().lines();
    let mut vec: Vec<char> = vec![];
    let mut n = 0;
    for line in input_content {
        let line_content = line.unwrap();
        for ch in line_content.chars() {
            vec.push(ch);
        }
        n += 1;
    }
    let mut mat = Array2::from_shape_vec((n, n), vec).unwrap();
    let start = mat.indexed_iter().find(|&((_, _), &c)| c == '^').unwrap().0;
    mat[start] = '.';
    return (mat, start);
}

fn visited_fields(mat: &Array2<char>, start: (usize, usize)) -> HashSet<Coord2D> {
    let mut pos: Coord2D = Coord2D::new(start);
    let mut dir: Coord2D = Coord2D(-1, 0);
    let mut visited: HashSet<Coord2D> = HashSet::new();
    loop {
        visited.insert(pos);
        let newpos = pos + dir;
        if mat.get(newpos.as_index()).is_none() {
            break; 
        }
        if mat[newpos.as_index()] == '.' {
            pos = newpos;
        } else {
            dir = Coord2D(dir.1, -dir.0);
        }
    }
    return visited;
}

fn part1(mat: &Array2<char>, start: (usize, usize)) -> usize {
    return visited_fields(mat, start).len();
}

fn is_infinite_loop(mat: &Array2<char>, start: (usize, usize), extra_block: (usize, usize)) -> bool {
    let mut pos: Coord2D = Coord2D::new(start);
    let mut dir: Coord2D = Coord2D(-1, 0);
    let mut visited: HashSet<(Coord2D, Coord2D)> = HashSet::new(); // pairs of pos and dir
    loop {
        if visited.contains(&(pos, dir)) {
            return true;
        }
        visited.insert((pos, dir));
        let newpos = pos + dir;
        if mat.get(newpos.as_index()).is_none() {
            return false;
        }
        if mat[newpos.as_index()] == '.' && newpos.as_index() != extra_block {
            pos = newpos;
        } else {
            dir = Coord2D(dir.1, -dir.0);
        }
    }
}

fn part2(mat: &Array2<char>, start: (usize, usize)) -> usize {
    let mut c = 0;
    for coord in visited_fields(mat, start) {
        if is_infinite_loop(&mat, start, coord.as_index()) {
            c += 1;
        }
    }
    return c;
}

fn main() {
    let (mat, start) = preprocess();
    let x = part1(&mat, start);
    println!("{}", x);
    let y = part2(&mat, start);
    println!("{}", y);
}
