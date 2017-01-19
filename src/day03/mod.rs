use std::error::Error;

fn parse_line(line: &str) -> Result<Vec<u32>, Box<Error>> {
    let numbers = line.split(" ")
    .filter(|s| !s.is_empty())
    .map(|s| s.parse::<u32>())
    .collect::<Result<Vec<_>, _>>()?;

    if numbers.len() != 3 {
        Err("bad line")?;
    }

    Ok(numbers)
}

#[allow(dead_code)]
pub mod part1 {
    use std::error::Error;
    use util;

    pub fn main() {
        match main_result() {
            Ok(total) => println!("Total: {}", total),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn main_result() -> Result<u32, Box<Error>> {
        let buf = util::read_file("inputs/day03/input.txt")?;
        let total = buf.lines()
        .map(parse_and_verify)
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .fold(0, |acc, &valid| acc + valid as u32);
        Ok(total)
    }

    fn parse_and_verify(line: &str) -> Result<bool, Box<Error>> {
        let mut numbers = super::parse_line(line)?;
        numbers.sort();
        Ok(numbers[0] + numbers[1] > numbers[2])
    }
}

#[allow(dead_code)]
pub mod part2 {
    use std::error::Error;
    use util;

    pub fn main() {
        match main_result() {
            Ok(total) => println!("Total: {}", total),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn main_result() -> Result<u32, Box<Error>> {
        let buf = util::read_file("inputs/day03/input.txt")?;
        let mut lines = buf.lines();
        let mut three_lines_res;
        let mut total = 0;
        loop {
            three_lines_res = get_three_lines(&mut lines);
            match three_lines_res {
                Ok(three_lines) => {
                    total += count_three_lines(&three_lines)?;
                },
                Err(_) => break,
            }
        }
        Ok(total)
    }

    fn count_three_lines(lines: &Vec<&str>) -> Result<u32, Box<Error>> {
        let mut numbers: [[u32; 3]; 3] = [[0; 3]; 3];
        let mut i = 0;
        for line in lines {
            let line_numbers = super::parse_line(line)?;
            numbers[0][i] = line_numbers[0];
            numbers[1][i] = line_numbers[1];
            numbers[2][i] = line_numbers[2];
            i += 1;
        }
        let mut total = 0;
        for triangle in numbers.iter() {
            let mut triangle_vec = triangle.to_vec();
            triangle_vec.sort();
            if triangle_vec[0] + triangle_vec[1] > triangle_vec[2] {
                total += 1;
            }
        }
        Ok(total)
    }

    fn get_three_lines<'a, T>(mut iter: T) -> Result<Vec<&'a str>, Box<Error>>
    where T: Iterator<Item=&'a str> {
        let a = iter.next().ok_or("EOF")?;
        let b = iter.next().ok_or("EOF")?;
        let c = iter.next().ok_or("EOF")?;
        Ok(vec![a, b, c])
    }
}
