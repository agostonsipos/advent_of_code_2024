use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Clone, Copy)]
struct Antenna {
    freq: char,
    pos: (i32, i32),
}

fn preprocess() -> (Vec<Antenna>, i32) {
    let stdin = io::stdin();
    let input_content = stdin.lock().lines();
    let mut vec: Vec<Antenna> = vec![];
    let mut n = 0;
    for line in input_content {
        let mut m = 0;
        let line_content = line.unwrap();
        for ch in line_content.chars() {
            if ch != '.' {
                vec.push(Antenna{freq: ch, pos: (n, m)});
            }
            m += 1;
        }
        n += 1;
    }
    return (vec, n);
}

fn display(antinodes: &HashSet<(i32,i32)>, size: i32) {
    for i in 0..size {
        for j in 0..size {
            if antinodes.contains(&(i,j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn count_antinodes(antennas: &Vec<Antenna>, size: i32, one_step: bool) -> usize {
    let in_bounds = |&(x, y) : &(i32, i32)| 0 <= x && x < size && 0 <= y && y < size;
    let mut antinodes: HashSet<(i32,i32)> = HashSet::new();
    for i in 0..antennas.len() {
        for j in i+1..antennas.len() {
            let a1 = antennas[i];
            let a2 = antennas[j];
            if a1.freq == a2.freq {
                let p1 = a1.pos;
                let p2 = a2.pos;
                let dx = p1.0 - p2.0;
                let dy = p1.1 - p2.1;
                let range: Box<dyn Iterator<Item = i32>> = if one_step { Box::new(1..=1) } else { Box::new(0..) };
                for k in range {
                    let p1 = (p1.0 + k * dx, p1.1 + k * dy);
                    let mut cont = false;
                    if in_bounds(&p1) {
                        antinodes.insert(p1);
                        cont = true;
                    }
                    let p2 = (p2.0 - k * dx, p2.1 - k * dy);
                    if in_bounds(&p2) {
                        antinodes.insert(p2);
                        cont = true;
                    }
                    if !cont {
                        break;
                    }
                }
            }
        }
    }
    // display(&antinodes, size);
    return antinodes.into_iter().filter(&in_bounds).count();
}

fn part1(antennas: &Vec<Antenna>, size: i32) -> usize {
    return count_antinodes(antennas, size, true);
}

fn part2(antennas: &Vec<Antenna>, size: i32) -> usize {
    return count_antinodes(antennas, size, false);
}

fn main() {
    let (antennas, size) = preprocess();
    let x = part1(&antennas, size);
    println!("{}", x);
    let y = part2(&antennas, size);
    println!("{}", y);
}
