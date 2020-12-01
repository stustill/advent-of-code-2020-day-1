use std::fs::File;
use std::{collections::HashSet, io::prelude::*};

fn main() -> std::io::Result<()> {
    let numbers = read_input()?;
    let result = find_numbers_totalling(&numbers, 2, 2020);
    println!("{:?}", result.unwrap());
    Ok(())
}

fn find_numbers_totalling(numbers: &HashSet<i64>, number: i16, total: i64) -> Option<i64> {
    numbers.iter().find_map(|num| {
        if number == 2 {
            numbers.get(&(total - num)).cloned()
        } else {
            find_numbers_totalling(numbers, number - 1, total - num)
        }
        .map(|com| num * com)
    })
}

fn read_input() -> std::io::Result<HashSet<i64>> {
    let mut input_file = File::open("input.txt")?;
    let mut input = String::new();
    let _ = input_file.read_to_string(&mut input)?;
    let numbers: HashSet<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    std::io::Result::Ok(numbers)
}
