use std::collections::{HashSet, HashMap};
const INPUT_DATA: &str = include_str!("../../data/input_day_4.txt");
fn part1(input: &str) -> u32 {
    input
    .lines()
    .fold(
        0,
        |acc, line| {
            let (_, contents) = line
                .split_once(":").expect("wrong format (no ':' )");

            let (winners, scratched) = contents
                .split_once("|").expect("wrong format (no '|' )");

            let winners = winners
                .trim()
                .split_whitespace()
                .fold(
                    HashSet::new(),
                    |mut set, winner| {
                        set.insert(winner.trim().parse::<u32>().expect("could not parse"));
                        set
                    }
                );

            let coincidences = scratched
                .trim()
                .split_whitespace()
                .fold(
                    0,
                    |mut acc, num| {
                        let num = num.trim().parse::<u32>().expect("could not parse");
                        if winners.contains(&num) {
                            acc += 1;
                        }
                        acc
                });

            acc + match coincidences {
                0 => 0,
                other => 2_u32.pow(other - 1),
            }
        }
    )
}

fn part2(input: &str) -> u32 {
    let scratchcards = input
    .lines()
    .fold(
        HashMap::new(),
        |mut winnings, line| {
        let (card_number, contents) = line
            .split_once(":")
            .expect("wrong format (no ':')");
        let card_number = card_number
            .split_whitespace()
            .last()
            .expect("no card_number")
            .parse::<u32>()
            .expect("could not parse card number");
        let card_multiplier = *winnings.entry(card_number).or_insert(1);
        let (winners, choosen) = contents
            .split_once("|")
            .expect("wrong format (no '|' )");
        let winners = winners
            .trim()
            .split_whitespace()
            .fold(
                HashSet::new(),
                |mut set, winner| {
                    set.insert(winner.trim().parse::<u32>().expect("could not parse"));
                    set
                }
            );
        let coincidences = choosen
            .trim()
            .split_whitespace()
            .fold(
                0,
                |mut acc, num| {
                    let num = num.trim().parse::<u32>().expect("could not parse");
                    if winners.contains(&num) {
                        acc += 1;
                    }
                    acc
            });
        for offset in 1..=coincidences {
            *winnings.entry(card_number + offset).or_insert(1_u32) += card_multiplier;
        }
        winnings
    });
    scratchcards.values().sum()
}
fn main() {
    let sum = part1(INPUT_DATA);
    println!("{}", sum);
    println!("{}", part2(INPUT_DATA));
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../data/test_day_4.txt");
    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 30);
    }
}