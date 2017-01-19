use std::error::Error;

#[derive(Debug)]
pub struct Room<'a> {
    name: &'a str,
    sector: u32,
    checksum: &'a str,
}

pub fn parse_line(line: &str) -> Result<Room, Box<Error>> {
    let mut bracket_split = line.split('[');
    let first_part = bracket_split.next().ok_or("Bad parse")?;
    let checksum = bracket_split.next().ok_or("Bad parse")?;

    let dash_index = first_part.rfind('-').ok_or("Bad parse")?;
    let (name, sector) = first_part.split_at(dash_index);
    let sector_int = sector[1..].parse::<u32>()?;
    Ok(Room { name: name, sector: sector_int, checksum: &checksum[..5] })
}

pub mod part1 {
    use util;
    use std::error::Error;
    use super::Room;

    pub fn main() {
        match main_error() {
            Ok(_) => {},
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    fn main_error() -> Result<(), Box<Error>> {
        let input = util::read_file("inputs/day04/input.txt")?;
        let lines = input.lines();
        let mut total = 0;
        let mut room;
        for line in lines {
            room = super::parse_line(line)?;
            if check_checksum(&room) {
                total += room.sector;
            }
        }
        println!("{}", total);
        Ok(())
    }

    fn check_checksum(room: &Room) -> bool {
        let top_counts = get_top_counts(&count_letters(room.name));
        let mut computed = String::new();
        for letter_code in top_counts[..5].iter() {
            computed.push((*letter_code + 97) as u8 as char);
        }
        computed == room.checksum
    }

    fn count_letters(name: &str) -> [u32; 26] {
        let mut counts = [0; 26];
        for letter in name.as_bytes().iter() {
            if *letter >= 97 && *letter <= 122 {
                counts[(*letter as usize) - 97] += 1;
            }
        }
        counts
    }

    fn get_top_counts(counts: &[u32; 26]) -> [usize; 26] {
        let mut count_tups = vec![];
        let mut i = 0;
        while i < 26 {
            count_tups.push((counts[i], 0 - (i as i32)));
            i += 1;
        }
        count_tups.sort();
        count_tups.reverse();
        let mut indices = [100; 26];
        i = 0;
        for count_tup in count_tups {
            indices[i] = (0 - count_tup.1) as usize;
            i += 1;
        }
        indices
    }
}
