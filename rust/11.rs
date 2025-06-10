use std::io::{self, Read};
use std::collections::HashMap;


fn preprocess() -> Vec<u64> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let _res = stdin.lock().read_to_string(&mut buffer);
    let stones: Vec<u64> = buffer.split(" ").map(|s| s.parse::<u64>().unwrap()).collect();
    return stones;
}

fn blink(stones: Vec<u64>) -> Vec<u64> {
    let mut new_stones: Vec<u64> = vec![];
    for st in stones {
        if st == 0 {
            new_stones.push(1);
        } else {
            let st_str = st.to_string();
            let l = st_str.len();
            if l % 2 == 0 {
                new_stones.push(st_str[0..l/2].parse::<u64>().unwrap());
                new_stones.push(st_str[l/2..l].parse::<u64>().unwrap());
            } else {
                new_stones.push(st * 2024);
            }
        }
    }
    return new_stones;
}

fn part1(sts: &Vec<u64>) -> usize {
    let mut stones = sts.clone();
    for _ in 0..25 {
        stones = blink(stones);
    }
    return stones.len();
}

// removes elements from the list, where the cache can tell how long the subsequence generated from them during the remaining steps will be
// also returns the sum of those subsequences
fn cleanup(sts: Vec<u64>, cache: &HashMap<(u64, u64), usize>, rem_steps: u64) -> (Vec<u64>, usize) {
    let mut new_stones: Vec<u64> = vec![];
    let mut accumulated = 0;
    for st in sts {
        let maybe_cached = cache.get(&(st, rem_steps));
        if maybe_cached.is_some() {
            accumulated += maybe_cached.unwrap();
        } else {
            new_stones.push(st);
        }
    }
    return (new_stones, accumulated);
}

fn part2(sts: &Vec<u64>, cache: &HashMap<(u64, u64), usize>, steps: u64) -> usize {
    let mut stones = sts.clone();
    let mut x = 0;
    for i in 1..=steps {
        stones = blink(stones);
        let (new_stones, y) = cleanup(stones, cache, steps - i);
        stones = new_stones;
        x += y;
    }
    return x + stones.len();
}

// creates a cache for all one-digit numbers about how long a sequence they generate in a given number of steps
fn compute_cache() -> HashMap<(u64, u64), usize> {
    let mut cache = HashMap::<(u64, u64), usize>::new();
    let n = 75;
    for j in 0..=n {
        for i in 0..=9 {
            let stones = vec![i];
            cache.insert((i, j), part2(&stones, &cache, j));
        }
    }
    return cache;
}

fn main() {
    let stones = preprocess();
    let x = part1(&stones);
    println!("{}", x);
    let cache = compute_cache();
    let y = part2(&stones, &cache, 75);
    println!("{}", y);
}