use anyhow::Result;
use std::str::Lines;

fn number_to_string(num: &str) -> &str {
    match num {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "1" => "1",
        "2" => "2",
        "3" => "3",
        "4" => "4",
        "5" => "5",
        "6" => "6",
        "7" => "7",
        "8" => "8",
        "9" => "9",
        _ => panic!("unknown string"),
    }
}

fn main() -> Result<()> {
    let p1_res = part1(include_str!("input.txt").lines())?;
    let p2_res = part2(include_str!("input.txt").lines())?;

    println!("Part 1: {p1_res}");
    println!("Part 2: {p2_res}");
    Ok(())
}

fn part1(lines: Lines) -> Result<usize> {
    let mut total = 0;
    for line in lines {
        // find all digits in the string
        // add together the first & last one
        // convert to a number, add to total
        let mut digits: Vec<&str> = Vec::new();
        for char in line.split("") {
            if let Ok(_) = char.parse::<usize>() {
                digits.push(char)
            }
        }
        let first_digit = digits[0];
        let last_digit = digits[digits.len() - 1];
        let combined = first_digit.to_owned() + last_digit;

        total += combined.parse::<usize>()?;
    }

    Ok(total)
}

fn part2(lines: Lines) -> Result<usize> {
    let mut total = 0;
    for line in lines {
        let patterns = vec![
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ];

        let mut matches: Vec<(usize, &str)> = vec![];
        for pattern in patterns {
            let pattern_matches = line.match_indices(pattern);
            for line_match in pattern_matches {
                matches.push((line_match.0, number_to_string(line_match.1)));
            }
        }

        // Sort by index found at
        matches.sort_by(|a, b| a.0.cmp(&b.0));

        let digits: Vec<&str> = matches.iter().map(|m| m.1).collect();

        let first_digit = digits[0];
        let last_digit = digits[digits.len() - 1];
        let combined = first_digit.to_owned() + last_digit;

        total += combined.parse::<usize>()?;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PT1_INPUT: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    const PT2_INPUT: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(PT1_INPUT.lines())?, 142);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(PT2_INPUT.lines())?, 281);
        Ok(())
    }
}
