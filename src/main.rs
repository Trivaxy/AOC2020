use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input_day6.txt").unwrap();
    println!("{}", day_6(&input, true));
}

///
/// DAY 1
///

fn day_1(input: &str, second_part: bool) -> i32 {
    let numbers = input.split('\n')
        .map(|l| i32::from_str(l).unwrap())
        .collect::<Vec<i32>>();

    for x in &numbers {
        for y in &numbers {
            if second_part {
                for z in &numbers {
                    if x + y + z == 2020 {
                        return x * y * z;
                    }
                }
            }
            else {
                if x + y == 2020 {
                    return x + y;
                }
            }
        }
    }

    unreachable!();
}

///
/// DAY 2
///

struct PasswordRule {
    min: usize,
    max: usize,
    char: char,
    password: String
}

impl PasswordRule {
    fn parse(input: &str) -> Self {
        let parts = input.split(' ').collect::<Vec<&str>>();

        let bounds = parts[0]
            .split('-')
            .map(|n| usize::from_str(&n).unwrap())
            .collect::<Vec<usize>>();

        let (min, max) = (bounds[0], bounds[1]);
        let char = parts[1].chars().next().unwrap();
        let password = parts[2];

        PasswordRule { min, max, char, password: password.to_owned() }
    }

    fn valid_1(&self) -> bool {
        let matches = self.password.matches(self.char).count();

        if matches >= self.min && matches <= self.max {
            return true;
        }

        false
    }

    fn valid_2(&self) -> bool {
        let password = self.password.chars().collect::<Vec<char>>();

        // it works so im keeping it this way, im lazy
        if (password[self.min - 1] == self.char && password[self.max - 1] != self.char)
            || (password[self.min - 1] != self.char && password[self.max - 1] == self.char) {
            return true;
        }

        false
    }
}

fn day_2(input: &str, second_part: bool) -> i32 {
    input.split('\n')
        .map(|l| {
            let password = PasswordRule::parse(l);
            if second_part {
                return password.valid_2();
            }
            password.valid_1()
        })
        .filter(|v| *v == true)
        .count() as i32
}

///
/// DAY 3
///

fn day_3(input: &str, xspeed: usize, yspeed: usize) -> i32 {
    let map = input.split('\n')
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    while y < map.len() - 1 {
        x += xspeed;
        y += yspeed;

        if x >= map[y].len() {
            x -= map[y].len();
        }

        if map[y][x] == '#' {
            trees += 1;
        }
    }

    trees
}

///
/// DAY 4
///

fn day_4(input: &str, second_part: bool) -> i32 {
    let raw_passports = input.split("\n\r")
        .map(|s| s.trim().to_owned())
        .map(|s| s.replace("\r\n", " "))
        .collect::<Vec<String>>();

    let mut passports = raw_passports.iter()
        .map(|p| p.split(|c| c == ' ' || c == ':').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let required_keys = [
        "byr", "iyr", "eyr", "hgt",
        "hcl", "ecl", "pid"
    ];

    passports.retain(|entry| {
        for key in &required_keys {
            if !entry.contains(key) {
                return false;
            }
        }
        true
    });

    if !second_part {
        return passports.len() as i32;
    }

    passports.retain(|entry| {
        let mut valid_key = false;

        for i in (0..entry.len()).step_by(2) {
            let key = entry[i];
            let value = entry[i + 1];

            valid_key = match key {
                "byr" => match i32::from_str(value) {
                    Ok(n) => n >= 1920 && n <= 2002,
                    Err(_) => false
                },
                "iyr" => match i32::from_str(value) {
                    Ok(n) => n >= 2010 && n <= 2020,
                    Err(_) => false
                },
                "eyr" => match i32::from_str(value) {
                    Ok(n) => n >= 2020 && n <= 2030,
                    Err(_) => false,
                },
                "hgt" => {
                    let n = match i32::from_str(&value[..value.len() - 2]) {
                        Ok(n) => n,
                        Err(_) => return false,
                    };

                    if value.ends_with("cm") {
                        n >= 150 && n <= 193
                    } else {
                        n >= 59 && n <= 76
                    }
                },
                "hcl" => value.len() == 7
                    && value.starts_with("#")
                    && value.chars().skip(1).all(|c| c.is_numeric() || (c >= 'a' && c <= 'f')),
                "ecl" => match value {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false,
                },
                "pid" => value.len() == 9 && i32::from_str(value).is_ok(),
                "cid" => true,
                _ => unreachable!()
            };

            if !valid_key {
                return false;
            }
        }

        true
    });

    passports.len() as i32
}

///
/// DAY 5
///

fn day_5(input: &str, second_part: bool) -> i32 {
    let passes = input.split('\n')
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut highest_id = 0;
    let mut seat_ids = Vec::new();

    for pass in &passes {
        let (mut min_row, mut max_row) = (0, 128);
        let (mut min_col, mut max_col) = (0, 8);

        for char in pass {
            match char {
                'F' => max_row = (max_row + min_row) / 2,
                'B' => min_row = (max_row + min_row) / 2,
                'R' => min_col = (max_col + min_col) / 2,
                'L' => max_col = (max_col + min_col) / 2,
                _ => unreachable!()
            }
        }

        let id = (max_row - 1) * 8 + (max_col - 1);
        seat_ids.push(id);

        if id > highest_id {
            highest_id = id;
        }
    }

    if !second_part {
        return highest_id;
    }

    let mut correct_id = 0;
    seat_ids.sort();

    for i in 1..seat_ids.len() {
        if seat_ids[i] - seat_ids[i - 1] != 1 {
            correct_id = (seat_ids[i] + seat_ids[i - 1]) / 2;
        }
    }

    correct_id
}

///
/// DAY 6
///

fn day_6(input: &str, second_part: bool) -> i32 {
    let groups = input.split("\r\n\r\n")
        .map(|g| g.trim()
            .split("\r\n")
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>())
        .collect::<Vec<Vec<Vec<char>>>>();

    let mut count = 0;

    for group in &groups {
        let mut answered_questions = HashMap::new();

        for person in group {
            if !second_part {
                for answer in person {
                    answered_questions.insert(answer, true);
                }
            } else {
                for other_person in group {
                    for answer in person {
                        if !answered_questions.contains_key(answer) || answered_questions[answer] == true {
                            if other_person.contains(answer) {
                                answered_questions.insert(answer, true);
                            } else {
                                answered_questions.insert(answer, false);
                            }
                        }
                    }
                }
            }
        }

        if !second_part {
            count += answered_questions.len() as i32;
        } else {
            count += answered_questions.iter()
                .filter(|&(k, v)| *v == true)
                .collect::<HashMap<&&char, &bool>>()
                .len() as i32;
        }
    }

    count
}