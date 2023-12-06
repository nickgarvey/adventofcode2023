use nom::{
    bytes::complete::{tag, take_until},
    character::complete::u64,
    combinator::opt,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Range {
    destination: u64,
    source: u64,
    length: u64,
}

#[derive(Debug)]
struct CategoryMap<'input_str> {
    source_name: &'input_str str,
    destination_name: &'input_str str,
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Almanac<'input_str> {
    seeds: Vec<u64>,
    // source -> map
    maps: HashMap<&'input_str str, CategoryMap<'input_str>>,
    _memo: HashMap<(&'input_str str, u64), u64>,
}

struct LocationIterator<'a> {
    almanac: &'a Almanac<'a>,

    location: &'a str,
    position: u64,
}

impl<'a> Iterator for LocationIterator<'a> {
    type Item = (&'a str, u64);

    fn next(&mut self) -> Option<(&'a str, u64)> {
        let (location, position) = self.almanac.next_hop(self.location, self.position)?;
        self.location = location;
        self.position = position;
        Some((location, position))
    }
}

impl<'a> Almanac<'_> {
    fn new(seeds: Vec<u64>, maps: HashMap<&'a str, CategoryMap<'a>>) -> Almanac<'a> {
        Almanac {
            seeds,
            maps,
            _memo: HashMap::new(),
        }
    }

    fn make_hop_iter(&'a self, location: &'a str, position: u64) -> LocationIterator {
        LocationIterator {
            almanac: self,
            location,
            position,
        }
    }

    fn next_hop(&self, source: &str, identifier: u64) -> Option<(&str, u64)> {
        let source_category_map = self.maps.get(source)?;

        let destination = source_category_map.destination_name;

        for range in source_category_map.ranges.iter() {
            if identifier >= range.source && identifier < range.source + range.length {
                return Some((destination, identifier + range.destination - range.source));
            }
        }
        return Some((destination, identifier));
    }

    fn find_min_location(&self, seeds: &[u64]) -> Result<u64, String> {
        let mut min_location = u64::MAX;

        for seed in seeds.iter() {
            match self.make_hop_iter("seed", *seed).last() {
                Some((category, position)) => {
                    if category != "location" {
                        return Result::Err(format!(
                            "Failed to find destination for seed {}",
                            *seed
                        ));
                    }
                    min_location = min_location.min(position);
                }
                None => {
                    return Result::Err(format!("Failed to find destination for seed {}", *seed));
                }
            }
        }

        Ok(min_location)
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (remaining, _) = tag("seeds: ")(input)?;
    let (remaining, seeds) = separated_list0(tag(" "), u64)(remaining)?;
    let (remaining, _) = tag("\n\n")(remaining)?;

    Ok((remaining, seeds))
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (remaining, (destination, _, source, _, length)) =
        tuple((u64, tag(" "), u64, tag(" "), u64))(input)?;

    Ok((
        remaining,
        Range {
            destination,
            source,
            length,
        },
    ))
}

fn parse_category_map(input: &str) -> IResult<&str, CategoryMap> {
    let (remaining, (source_name, _, destination_name, _)) = tuple((
        take_until("-"),
        tag("-to-"),
        take_until(" "),
        tag(" map:\n"),
    ))(input)?;

    let (remaining, ranges) = separated_list0(tag("\n"), parse_range)(remaining)?;
    let (remaining, _) = opt(tag("\n"))(remaining)?;

    Ok((
        remaining,
        CategoryMap {
            source_name,
            destination_name,
            ranges,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Almanac> {
    let (remaining, seeds) = parse_seeds(input)?;
    let (remaining, map_vec) = separated_list0(tag("\n"), parse_category_map)(remaining)?;

    let maps = HashMap::from_iter(map_vec.into_iter().map(|map| (map.source_name, map)));

    Ok((remaining, Almanac::new(seeds, maps)))
}

pub fn part1(input_path: &str) -> Result<u64, String> {
    let input = fs::read_to_string(input_path).unwrap();
    let (remaining, almanac) =
        parse(&input).map_err(|e| format!("Failed to parse input: {}", e))?;
    assert_eq!("", remaining);

    almanac.find_min_location(almanac.seeds.as_slice())
}

pub fn part2(input_path: &str) -> Result<u64, String> {
    let input = fs::read_to_string(input_path).unwrap();
    let (remaining, almanac) =
        parse(&input).map_err(|e| format!("Failed to parse input: {}", e))?;
    assert_eq!("", remaining);

    let mut min_location = u64::MAX;
    // iterate over all the pairs of seeds
    for chunk in almanac.seeds.chunks_exact(2) {
        let start = chunk[0];
        let length = chunk[1];

        let range = start..start + length;
        let slice: Vec<u64> = range.collect();

        let location = almanac.find_min_location(&slice[..])?;
        min_location = min_location.min(location);
    }
    Ok(min_location)
}
