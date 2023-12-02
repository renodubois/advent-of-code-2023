use anyhow::Result;
use std::str::Lines;

fn main() -> Result<()> {
    let p1_res = part1(include_str!("input.txt").lines())?;
    let p2_res = part2(include_str!("input.txt").lines())?;

    println!("Part 1: {p1_res}");
    println!("Part 2: {p2_res}");
    Ok(())
}

fn color_to_index(color: &str) -> usize {
    match color {
        "red" => 0,
        "green" => 1,
        "blue" => 2,
        _ => panic!("unexpected color"),
    }
}

fn part1(lines: Lines) -> Result<usize> {
    let mut total = 0;
    let total_cubes = [12, 13, 14];
    for line in lines {
        let mut id: usize = 0;
        let mut is_valid = true;
        for part in line.split(": ") {
            if part.contains("Game") {
                id = part.trim_start_matches("Game ").parse::<usize>()?;
            } else {
                // the game results
                for pull in part.split("; ") {
                    for res in pull.split(", ") {
                        let split_res: Vec<&str> = res.split(" ").collect();
                        let num = split_res[0].parse::<usize>()?;
                        let color = split_res[1];
                        if num > total_cubes[color_to_index(color)] {
                            is_valid = false;
                        }
                    }
                }
            }
        }

        if is_valid == true {
            total += id;
        }
    }
    Ok(total)
}

fn part2(lines: Lines) -> Result<usize> {
    // keep a running max of the colors from the results
    let mut total = 0;
    for line in lines {
        let mut max_cubes = [0, 0, 0];
        for part in line.split(": ") {
            if !part.contains("Game") {
                // the game results
                for pull in part.split("; ") {
                    for res in pull.split(", ") {
                        let split_res: Vec<&str> = res.split(" ").collect();
                        let num = split_res[0].parse::<usize>()?;
                        let color = split_res[1];
                        let index = color_to_index(color);
                        if num > max_cubes[index] {
                            max_cubes[index] = num;
                        }
                    }
                }
            }
        }

        let power: usize = max_cubes.iter().product();
        total += power;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(include_str!("pt1_example.txt").lines())?, 8);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(include_str!("pt2_example.txt").lines())?, 2286);
        Ok(())
    }
}
