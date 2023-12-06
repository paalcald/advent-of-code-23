use std::{iter::zip};

const INPUT_DATA: &str = include_str!("../../data/input_day_6.txt");

fn possible_improvements(time: u64, distance: u64) -> u64 {
    let t = time as f32;
    let d = distance as f32;
    let lower_sol = (t - (t * t - 4.0 * d).sqrt()) / 2.0;
    let upper_sol = (t + (t * t - 4.0 * d).sqrt()) / 2.0;
    //Since we look for greater, not greater or equal we have to do this
    let lower_bound = if lower_sol.ceil() == lower_sol {
        lower_sol.ceil() as u64 + 1
    } else {
        lower_sol.ceil() as u64
    };
    let upper_bound = if upper_sol.floor() == upper_sol {
        (upper_sol.floor() as u64) - 1
    } else {
        upper_sol.floor() as u64
    };
    dbg!(lower_bound, upper_bound);
    upper_bound - lower_bound + 1
}

macro_rules! parse_input {
    ($input:ident) => {    
    $input
        .next()
        .expect("unvalid input")
        .split_once(":")
        .expect("unvalid format")
        .1
        .trim()
        .split_whitespace()
        .map(|s| {
            s
                .trim()
                .parse()
                .expect("nums")
            })
    };
}
fn part1(input: &str) -> u64 {
    let mut input = input.lines();
    let times = parse_input!(input);
    let distances = parse_input!(input);
    let races = zip(times, distances);

    races.fold(1, |acc, (time, distance)| {
        acc * possible_improvements(time, distance)
    })
}
fn part2(input: &str) -> u64 {
    part1(input.replace(" ", "").as_str())
}
fn main() {
    //println!("{}", part1(INPUT_DATA));
    println!("{}", part2(INPUT_DATA));
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../data/test_day_6.txt");
    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 288);
    }
    #[test]
    fn ceil_of_natural() {
        assert_eq!(1_f32.ceil(), 2.0)
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 71503);
    }
}