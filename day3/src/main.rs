use anyhow::Result;
use regex::Regex;
use std::{collections::HashMap, str::Lines};

fn main() -> Result<()> {
    let p1_res = part1(include_str!("input.txt").lines())?;
    let p2_res = part2(include_str!("input.txt").lines())?;

    println!("Part 1: {p1_res}");
    println!("Part 2: {p2_res}");
    Ok(())
}

const DIR: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (1, -1),
    (1, 1),
    (-1, 0),
    (-1, 1),
    (-1, -1),
];

fn part1(lines: Lines) -> Result<usize> {
    let mut total = 0;

    // make an x,y set with every space that is next to a symbol
    // then, find all of the numbers, compare their x,y to that set
    // if they intersect, that's a valid part number
    let mut valid_spaces: HashMap<(usize, usize), bool> = HashMap::new();

    for (x, line) in lines.clone().into_iter().enumerate() {
        for (y, part) in line.chars().enumerate() {
            if part != '.' {
                if let Ok(_) = part.to_string().parse::<usize>() {
                    // number, ignoring for now
                } else {
                    // symbol
                    for (xmod, ymod) in DIR {
                        let new_x = x.checked_add_signed(xmod);
                        let new_y = y.checked_add_signed(ymod);

                        if new_x.is_some() && new_y.is_some() {
                            valid_spaces.insert((new_x.unwrap(), new_y.unwrap()), true);
                        }
                    }
                }
            }
        }
    }

    for (x, line) in lines.into_iter().enumerate() {
        let re = Regex::new(r"\d+").unwrap();
        for re_match in re.find_iter(line) {
            for y in re_match.start()..re_match.end() {
                if valid_spaces.contains_key(&(x, y)) {
                    total += re_match.as_str().parse::<usize>()?;
                    break;
                }
            }
        }
    }

    Ok(total)
}

fn part2(lines: Lines) -> Result<usize> {
    let mut total = 0;

    let mut valid_spaces: HashMap<(usize, usize), usize> = HashMap::new();

    for (x, line) in lines.clone().into_iter().enumerate() {
        let re = Regex::new(r"\d+").unwrap();
        for re_match in re.find_iter(line) {
            let num = re_match.as_str().parse::<usize>()?;
            for y in re_match.start()..re_match.end() {
                valid_spaces.insert((x, y), num);
            }
        }
    }

    for (x, line) in lines.clone().into_iter().enumerate() {
        for (y, part) in line.chars().enumerate() {
            if part != '.' {
                if let Ok(_) = part.to_string().parse::<usize>() {
                    // number, ignoring for now
                } else if part == '*' {
                    // symbol
                    let mut nums: Vec<usize> = Vec::new();
                    for (xmod, ymod) in DIR {
                        let new_x = x.checked_add_signed(xmod);
                        let new_y = y.checked_add_signed(ymod);

                        if new_x.is_some() && new_y.is_some() {
                            if let Some(num) = valid_spaces.get(&(new_x.unwrap(), new_y.unwrap())) {
                                nums.push(num.to_owned());
                            }
                        }
                    }
                    nums.dedup();
                    if nums.len() == 2 {
                        total += nums.into_iter().reduce(|acc, i| acc * i).unwrap();
                    }
                }
            }
        }
    }

    // find all of the numbers, insert them into a hash map with (x,y) -> num
    // go look for *, check to see if they have two distinct numbers adjacent,
    // and if so, multiply numbers & add to total

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(include_str!("pt1_example.txt").lines())?, 4361);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(include_str!("pt2_example.txt").lines())?, 467835);
        Ok(())
    }
}
