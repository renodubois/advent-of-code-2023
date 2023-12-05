use anyhow::Result;
use std::{collections::HashMap, str::Lines};

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
        let mut card_total = 0;
        let mut winning_nums: Vec<usize> = Vec::new();

        let split_line: &str = line.split(": ").collect::<Vec<&str>>()[1];
        let line_nums = split_line.split(" | ");

        for (index, num_string) in line_nums.enumerate() {
            if index == 0 {
                // winning numbers
                for winning_num in num_string.split_whitespace() {
                    winning_nums.push(winning_num.parse::<usize>()?);
                }
            } else {
                for card_num in num_string.split_whitespace() {
                    let parsed_num = card_num.parse::<usize>()?;
                    if winning_nums.contains(&parsed_num) {
                        if card_total == 0 {
                            card_total = 1
                        } else {
                            card_total *= 2;
                        }
                    }
                }
            }
        }

        total += card_total;
    }

    Ok(total)
}

fn part2(lines: Lines) -> Result<usize> {
    let mut total = 0;
    let mut card_copies: HashMap<usize, usize> = HashMap::new();

    for line in lines {
        let mut card_total = 0;
        let mut winning_nums: Vec<usize> = Vec::new();

        let split_line: Vec<&str> = line.split(": ").collect();

        let card_no =
            split_line[0].split_whitespace().collect::<Vec<&str>>()[1].parse::<usize>()?;

        if !card_copies.contains_key(&card_no) {
            card_copies.insert(card_no, 1);
        }

        let line_nums = split_line[1].split(" | ");

        for (index, num_string) in line_nums.enumerate() {
            if index == 0 {
                // winning numbers
                for winning_num in num_string.split_whitespace() {
                    winning_nums.push(winning_num.parse::<usize>()?);
                }
            } else {
                for card_num in num_string.split_whitespace() {
                    let parsed_num = card_num.parse::<usize>()?;
                    if winning_nums.contains(&parsed_num) {
                        card_total += 1;
                    }
                }
            }
        }

        for new_card_no in (card_no + 1)..(card_no + 1 + card_total) {
            let current_copies = card_copies.get(&card_no).unwrap();
            if !card_copies.contains_key(&new_card_no) {
                // every card will have 1 copy, add an extra
                card_copies.insert(new_card_no, 1 + current_copies);
            } else {
                let cur_val = card_copies.get(&new_card_no).unwrap();
                card_copies.insert(new_card_no, cur_val + current_copies);
            }
        }
    }

    for (_, v) in card_copies.into_iter() {
        total += v;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(include_str!("pt1_example.txt").lines())?, 13);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(include_str!("pt2_example.txt").lines())?, 30);
        Ok(())
    }
}
