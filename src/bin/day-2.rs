use std::collections::HashMap;

const TEST_DATA: &str = include_str!("../../data/input_day_2.txt");


fn part1() {
    let max_cubes: HashMap<&str, u32> = [
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]
    .iter().cloned().collect();
    let imposible_cases = TEST_DATA.split("\n")
        .filter_map(|line| {
            if let Some((game_header, game_sets))  = line.split_once(":") {
                let (_, game_num_str) = game_header.trim().split_once(" ").expect("could not parse game header");
                let game_num = game_num_str.parse::<u32>().expect("unable to extract game number");
                let is_set_valid = game_sets.split(";")
                    .all(|set| {
                        set
                            .split(",")
                            .all(|freq_color| {
                                let (num, color) = freq_color.trim().split_once(" ").expect("could not extract color freq pair");
                                let current_color_max = max_cubes.get(color).expect("no cubes of that color");
                                let current_color_cubes = num.parse::<u32>().expect("number of colors is not a number");
                                if current_color_max >= &current_color_cubes {
                                    true
                                } else {
                                    false
                                }
                            })
                    });
                if is_set_valid {
                    Some(game_num)
                } else {
                    None
                }
            } else {
                None
            }
        });
    let sol: u32 = imposible_cases.sum();
    println!("{}", sol);
}

fn part2() {
    todo!()
}
fn main() {
    part1();
    //part2();
}