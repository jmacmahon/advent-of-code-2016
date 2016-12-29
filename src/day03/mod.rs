use std::fs::File;
use std::io::Read;
use std::error::Error;

pub fn main() {
    match main_result() {
        Ok(total) => println!("Total: {}", total),
        Err(e) => println!("Error: {}", e),
    }
}

fn main_result() -> Result<u32, Box<Error>> {
    let mut f = File::open("inputs/day03/input.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let total = buf.lines()
        .map(|line| parse_and_verify(line))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .fold(0, |acc, &valid| acc + valid as u32);
    Ok(total)
}

fn parse_and_verify(line: &str) -> Result<bool, Box<Error>> {
    let mut numbers = line.split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;
    if numbers.len() != 3 {
        Err("bad line")?;
    }
    numbers.sort();
    Ok(numbers[0] + numbers[1] > numbers[2])
}
