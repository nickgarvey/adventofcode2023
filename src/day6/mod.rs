use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, u64},
    multi::separated_list0,
    sequence::tuple,
    Finish, IResult,
};
use std::{fmt::Display, fs, str::FromStr};

fn parse_times(input: &str) -> IResult<&str, Vec<u64>> {
    let (remain, _) = tag("Time:")(input)?;
    let (remain, _) = multispace1(remain)?;
    separated_list0(multispace1, u64)(remain)
}

fn parse_distances(input: &str) -> IResult<&str, Vec<u64>> {
    let (remain, _) = tag("Distance:")(input)?;
    let (remain, _) = multispace1(remain)?;
    separated_list0(multispace1, u64)(remain)
}

fn parse(input: &str) -> Result<(Vec<u64>, Vec<u64>), nom::error::Error<&str>> {
    let (remain, (times, _, distances)) =
        tuple((parse_times, tag("\n"), parse_distances))(input).finish()?;
    assert_eq!(remain, "");
    Ok((times, distances))
}

pub fn part1(input_path: &str) -> Result<u64, String> {
    let input = fs::read_to_string(input_path).unwrap();
    match parse(&input) {
        Ok((times, distances)) => {
            let races: Vec<_> = times.into_iter().zip(distances.into_iter()).collect();
            let mut product = 1u64;
            for (time, distance) in races.iter() {
                product *= (1..*time).filter(|t| (time - t) * t > *distance).count() as u64;
            }
            Ok(product)
        }
        Err(e) => Err(format!("Parse Error: {}", e)),
    }
}

fn vec_list_parse<F: FromStr, T: Display>(vec: Vec<T>) -> Result<F, F::Err> {
    vec.iter()
        .fold(String::from(""), |acc, t| acc + &t.to_string())
        .parse::<F>()
}

pub fn part2(input_path: &str) -> Result<u64, String> {
    let input = fs::read_to_string(input_path).map_err(|e| format!("File read error: {}", e))?;

    let (times, distances) = parse(&input).map_err(|e| format!("Parse Error: {}", e))?;

    let time: u64 = vec_list_parse(times).map_err(|e| format!("Number join error: {}", e))?;

    let distance: u64 =
        vec_list_parse(distances).map_err(|e| format!("Number join error: {}", e))?;

    Ok((1..time).filter(|t| (time - t) * t > distance).count() as u64)
}
