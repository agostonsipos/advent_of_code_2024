use std::io::{self, Read};


fn preprocess() -> Vec<Option<i32>> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let _res = stdin.lock().read_to_string(&mut buffer);
    let lengths: Vec<u32> = buffer.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut input = Vec::<Option<i32>>::new();
    let mut id = 0;
    let mut file = true;
    for length in lengths {
    	for _ in 0..length {
    		if file {
    			input.push(Some(id));
    		} else {
    			input.push(None);
    		}
    	}
    	file = !file;
    	if file {
    		id += 1;
    	}
    }
    return input;
}

fn checksum(data: &Vec<Option<i32>>) -> u64 {
	let mut x = 0;
	for i in 0..data.len() {
		if data[i].is_some() {
			x += i as u64 * data[i].unwrap() as u64;
		}
	}
	return x;
}

fn part1(mut data: Vec<Option<i32>>) -> u64 {
	let mut i = 0;
	let mut j = data.len() - 1;
	while i < j {
		if data[i].is_some() {
			i += 1;
		} else if data[j].is_none() {
			j -= 1;
		} else {
			data[i] = data[j];
			data[j] = None;
			i += 1;
			j -= 1;
		}
	}
	return checksum(&data);
}

fn part2(mut data: Vec<Option<i32>>) -> u64 {
	let mut i = 0;
	let mut j = data.len() - 1;
	while i < j {
		if data[i].is_some() {
			i += 1;
		} else if data[j].is_none() {
			j -= 1;
		} else {
			let value = data[j];
			let mut l = 0;
			while data[j-l].is_some() && data[j-l].unwrap() == value.unwrap() {
				l += 1;
			}
			let mut k = i;
			while k < j {
				if data[k..k+l].iter().all(|&x| x.is_none()) {
					data[k..k+l].fill(value);
					data[j-l+1..j+1].fill(None);
					break;
				}
				k += 1;
			}
			j -= l;
		}
	}
	return checksum(&data);
}



fn main() {
    let inp = preprocess();
    let x = part1(inp.clone());
    println!("{}", x);
    let y = part2(inp.clone());
    println!("{}", y);
}
