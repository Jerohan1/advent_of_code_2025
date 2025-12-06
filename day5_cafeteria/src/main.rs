use std::env::args;
use std::fs::read_to_string;

fn parse_database(input_data: Vec<&str>) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();

    for line in input_data {
        if line.contains("-") {
            let (a, b) = line.split_once("-").unwrap();
            (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap());
            ranges.push((a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()));
        } else if !line.is_empty() {
            ingredients.push(line.parse::<u64>().unwrap());
        }
    }
    return (ranges, ingredients);
}

fn number_fresh(ranges: &Vec<(u64, u64)>, ingredients: Vec<u64>) -> i32 {
    let mut n_fresh: i32 = 0;
    for ing in ingredients {
        for range in ranges {
            if (ing >= range.0) && (ing <= range.1) {
                n_fresh += 1;
                break;
            }
        }
    }
    return n_fresh;
}

fn number_ids(mut ranges: Vec<(u64, u64)>) -> u64 {
    ranges.sort();

    let mut merged: Vec<(u64, u64)> = Vec::new();

    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start < last.1 + 1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    let total = merged.iter().map(|(start, end)| end - start + 1).sum();
    return total;
}

fn main() {
    let args: Vec<String> = args().collect();
    let file_path: &String = &args[1];
    let input = read_to_string(file_path).expect("Something went wrong reading the file");
    let lines: Vec<&str> = input.lines().collect();

    let (ranges, ingredients) = parse_database(lines);
    let n_fresh = number_fresh(&ranges, ingredients);
    println!("Number of fresh ingredients: {n_fresh}");

    let n_ids = number_ids(ranges);
    println!("Number of IDS: {:?}", n_ids);
}
