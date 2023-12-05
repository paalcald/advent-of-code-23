use std::{collections::HashSet, usize};

const INPUT_DATA: &str = include_str!("../../data/input_day_3.txt");

enum GearRatio {
    None,
    One(u32),
    Ratio(u32)
}
fn part1(input: &str) -> u32 {
    let mut adjacent_to_symbol: HashSet<(usize, usize)> = HashSet::new();
    let mut found_numbers: Vec<(u32, Vec<(usize, usize)>)> = Vec::new();
    input
        .split("\n")
        .map(|x| x.trim())
        .enumerate()
        .for_each(|(row_n , column )| {
            let (num_buf, digits) = column
                .chars()
                .enumerate()
                .map(|(col_n, c)|
                    (c, (row_n, col_n)))
                .fold(
                    (None, Vec::new()),
                    |(mut num_buf, mut digits), x| {
                    let c = x.0;
                    let coords = x.1;
                    if c.is_digit(10) {
                        let n = c.to_digit(10).expect("is digit");
                        if let Some(num) = num_buf {
                            num_buf = Some(num * 10 + n);
                        } else {
                            num_buf = Some(n);
                        }
                        digits.push(coords);
                        (num_buf, digits)
                    } else {
                        if let Some(num) = num_buf {
                            found_numbers.push((num, digits));
                        }
                        if c != '.' {
                            let row = coords.0;
                            let col = coords.1;
                            let adjacents = [
                                (row - 1, col - 1), (row - 1, col), (row - 1, col + 1),
                                (row, col - 1), (row, col), (row, col + 1),
                                (row + 1, col - 1), (row + 1, col), (row + 1, col + 1)
                            ];
                            for adjacent in adjacents {
                                adjacent_to_symbol.insert(adjacent);
                            }
                        }
                        (None, Vec::new())
                    }
                });
                if let Some(num) = num_buf {
                    found_numbers.push((num, digits));
                }
        });
    let sum: u32 = found_numbers.iter().filter_map(|(num, digits)| {
        if digits.iter().any(|x| adjacent_to_symbol.contains(x)) {
            Some(num)
        } else {
            None
        }
    }).sum();
    sum
}

fn part2(input: &str) -> u32{
    let mut gears: HashSet<(usize, usize)> = HashSet::new();
    let mut found_numbers: Vec<(u32, Vec<(usize, usize)>)> = Vec::new();
    input
        .split("\n")
        .map(|x| x.trim())
        .enumerate()
        .for_each(|(row_n , column )| {
            let (num_buf, digits) = column
                .chars()
                .enumerate()
                .map(|(col_n, c)|
                    (c, (row_n, col_n)))
                .fold(
                    (None, Vec::new()),
                    |(mut num_buf, mut digits), x| {
                    let c = x.0;
                    let coords = x.1;
                    if c.is_digit(10) {
                        let n = c.to_digit(10).expect("is digit");
                        if let Some(num) = num_buf {
                            num_buf = Some(num * 10 + n);
                        } else {
                            num_buf = Some(n);
                        }
                        digits.push(coords);
                        (num_buf, digits)
                    } else {
                        if let Some(num) = num_buf {
                            found_numbers.push((num, digits));
                        }
                        if c == '*' {
                            gears.insert(coords);
                        }
                        (None, Vec::new())
                    }
                });
                if let Some(num) = num_buf {
                    found_numbers.push((num, digits));
                }
        });
    let mut sum = 0;
    for gear in gears {
        let mut ratio = GearRatio::None;
        for (number, digits) in &found_numbers {
            let mut neighborhood = HashSet::new();
            for digit in digits {
                let row = digit.0;
                let col = digit.1;
                let adjacents = [
                    (row.checked_sub(1), col.checked_sub(1)), (row.checked_sub(1), Some(col)), (row.checked_sub(1), Some(col + 1)),
                    (Some(row), col.checked_sub(1)), (Some(row), Some(col)), (Some(row), Some(col + 1)),
                    (Some(row + 1), col.checked_sub(1)), (Some(row + 1), Some(col)), (Some(row + 1), Some(col + 1))
                ];
                for adjacent in adjacents {
                    if let (Some(row), Some(col)) = adjacent {
                        neighborhood.insert((row, col));
                    }
                }
            }
            if neighborhood.contains(&gear) {
                ratio = match ratio {
                    GearRatio::None => {
                        GearRatio::One(*number)
                    },
                    GearRatio::One(only) => {
                        GearRatio::Ratio(only * number)
                    },
                    GearRatio::Ratio(prev) => {
                        GearRatio::Ratio(prev * number)
                    }
                }
            }
        }
        if let GearRatio::Ratio(ratio) = ratio {
            sum += ratio;
        }
    }
    sum
}

fn main() {
    println!("part 1 solution is {}", part1(INPUT_DATA));
    println!("part 2 solution is {}", part2(INPUT_DATA));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../data/test_day_3.txt");
    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 4361);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 467835);
    }
}
