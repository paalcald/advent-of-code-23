const INPUT_DATA: &str = include_str!("../../data/input_day_5.txt");

#[derive(Debug)]
struct InputData
where {
    seeds: Vec<SeedRange>,
    seed_to_soil_map: Vec<Range>,
    soil_to_fertilizer_map: Vec<Range>,
    fertilizer_to_water_map: Vec<Range>,
    water_to_light_map: Vec<Range>,
    light_to_temperature_map: Vec<Range>,
    temperature_to_humidity_map: Vec<Range>,
    humidity_to_location_map: Vec<Range>,
}

fn parse_pair(s: &str) -> Result<(SeedRange, &str), &str> {
    if s.trim().is_empty() {
        return Err("done")
    }
    if let Some((first, rest)) = s.split_once(" ") {
        if let Some((second, rest)) = rest.split_once(" ") {
            Ok(
                (SeedRange {
                    range_start: first.parse::<u64>().unwrap(),
                    range_len: second.parse::<u64>().unwrap(),
                 }, rest))
        } else {
            Ok(
                (SeedRange {
                    range_start: first.parse::<u64>().unwrap(),
                    range_len: rest.parse::<u64>().unwrap(),
                 }, ""))
        }
    } else {
        Err("no first")
    }
}

macro_rules! process_section {
    ($section_name:ident , $stream:ident) => {
        let section_str = format!("{}", stringify!($section_name).replace("_", "-").replace("-map", " map:"));
        assert_eq!(section_str, $stream.next().expect(section_str.as_ref()));
        let $section_name: Vec<Range> = $stream.by_ref()
            .take_while(|&line| !line.is_empty())
            .map(|range_str| range_str.into())
            .collect();
    };
}

impl From<&str> for InputData {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let (_, mut seeds_str) = lines
            .next()
            .expect("header line")
            .split_once(":")
            .expect("header is formated");
        let mut seeds = Vec::new();
        loop {
            match parse_pair(seeds_str.trim()) {
                Ok((range, rest)) => {
                    seeds.push(range);
                    seeds_str = rest;
                },
                Err(other) => {println!("{}", other); break}
            }
        }
        let seeds = seeds;
        let _ = lines.next();
        process_section!(seed_to_soil_map, lines);
        process_section!(soil_to_fertilizer_map, lines);
        process_section!(fertilizer_to_water_map, lines);
        process_section!(water_to_light_map, lines);
        process_section!(light_to_temperature_map, lines);
        process_section!(temperature_to_humidity_map, lines);
        process_section!(humidity_to_location_map, lines);
        Self {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            light_to_temperature_map,
            water_to_light_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        }
    }
}
#[derive(Debug)]
struct SeedRange {
    range_start: u64,
    range_len: u64,
}
struct SeedIterator {
    current: u64,
    last: u64
}
impl Iterator for SeedIterator {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.last {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}
impl IntoIterator for SeedRange {
    type Item = u64;
    type IntoIter = SeedIterator;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            current: self.range_start,
            last: self.range_start + self.range_len,
        }
    }
}

impl From<(u64, u64)> for SeedRange {
    fn from(value: (u64, u64)) -> Self {
        Self { range_start: value.0, range_len: value.1 }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    source_range_start: u64,
    source_range_end: u64,
    destination_range_start: u64,
}

impl Range {
    fn new(
        destination_range_start: u64,
        source_range_start: u64,
        range_length: u64) -> Self {
            Self {
                source_range_start,
                source_range_end: source_range_start + range_length - 1,
                destination_range_start, 
            }
        }
    fn contains(&self, other: u64) -> bool {
        self.source_range_start <= other && other <= self.source_range_end
    }
    fn get_value_for(&self, value: u64) -> Option<u64> {
        if self.contains(value) {
            Some(self.destination_range_start + value - self.source_range_start)
        } else {
            None
        }
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let mut values = value
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u64>().expect("should be nums"));
        let destination_range_start = values.next().expect("there be 1");
        let source_range_start = values.next().expect("there be 2");
        let range_length = values.next().expect("there be 3");
        Range::new(destination_range_start, source_range_start, range_length)
    }
}
fn part1(input : &str) -> u64 {
    todo!("When I implemented part2 I broke part one parser.
     Sorry, you can see it going to the previous commit");
}
fn part2(input : &str) -> u64 {
    let data = InputData::from(input);
    data.seeds
        .into_iter()
        .flat_map(|x|
            x.into_iter()
        )
        .map(|seed| {
            let mapped_value = data.seed_to_soil_map
                .iter()
                .find_map(|map| map.get_value_for(seed));
            match mapped_value{
                Some(value) => value,
                None => seed,
            }
        })
        .map(|seed| {
            let mapped_value = data.seed_to_soil_map
                .iter()
                .find_map(|map| map.get_value_for(seed));
            match mapped_value{
                Some(value) => value,
                None => seed,
            }
        })
        .map(|soil| {
            match data.soil_to_fertilizer_map
                .iter()
                .find_map(|map| map.get_value_for(soil)){
                    Some(value) => value,
                    None => soil
                }
        })
        .map(|fertilizer| {
            match data.fertilizer_to_water_map
                .iter()
                .find_map(|map| map.get_value_for(fertilizer)){
                    Some(value) => value,
                    None => fertilizer
                }
        })
        .map(|fertilizer| {
            match data.water_to_light_map
                .iter()
                .find_map(|map| map.get_value_for(fertilizer)){
                    Some(value) => value,
                    None => fertilizer
                }
        })
        .map(|fertilizer| {
            match data.light_to_temperature_map
                .iter()
                .find_map(|map| map.get_value_for(fertilizer)){
                    Some(value) => value,
                    None => fertilizer
                }
        })
        .map(|fertilizer| {
            match data.temperature_to_humidity_map
                .iter()
                .find_map(|map| map.get_value_for(fertilizer)){
                    Some(value) => value,
                    None => fertilizer
                }
        })
        .map(|fertilizer| {
            match data.humidity_to_location_map
                .iter()
                .find_map(|map| map.get_value_for(fertilizer)){
                    Some(value) => value,
                    None => fertilizer
                }
        })
        .min()
        .unwrap()
    //let min_location_index = location
      //  .into_iter()
      //  .enumerate()
      //  .min_by(|(_, a), (_ , b)| a.cmp(b))
      //  .map(| (index, _)| index);
}
fn main() {
    println!("{}", part2(INPUT_DATA));
   // println!("{}", part2(INPUT_DATA));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../data/test_day_5.txt");
    #[test]
    fn test_input_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(part2(TEST_INPUT), 30);
    }
}