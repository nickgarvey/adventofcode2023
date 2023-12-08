use clap::Parser;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

#[derive(Parser)]
#[command(name = "nickgarvey Advent of Code 2023")]
#[command(author = "Nick Garvey <garvey.nick@gmail.com>")]
struct Args {
    day: String,
    part: String,

    #[arg(short = 's', long)]
    use_sample_input: bool,
}

fn main() {
    let args = Args::parse();
    let day = args.day.as_str();
    let part = args.part.as_str();

    let path = format!(
        "src/{}/{}input.txt",
        day,
        if args.use_sample_input {
            args.part.clone() + "_sample_"
        } else {
            "".to_string()
        }
    );

    // check path exists
    if !std::path::Path::new(&path).exists() {
        println!("{} does not exist", path);
        return;
    }

    match (day, part) {
        ("day1", "part1") => println!("{}", day1::part1(&path)),
        ("day1", "part2") => println!("{}", day1::part2(&path)),
        ("day2", "part1") => println!("{}", day2::part1(&path)),
        ("day2", "part2") => println!("{}", day2::part2(&path)),
        ("day3", "part1") => println!("{}", day3::part1(&path)),
        ("day3", "part2") => println!("{}", day3::part2(&path)),
        ("day4", "part1") => println!("{}", day4::part1(&path)),
        ("day4", "part2") => println!("{}", day4::part2(&path)),
        ("day5", "part1") => println!("{}", day5::part1(&path).unwrap()),
        ("day5", "part2") => println!("{}", day5::part2(&path).unwrap()),
        ("day6", "part1") => println!("{}", day6::part1(&path).unwrap()),
        ("day6", "part2") => println!("{}", day6::part2(&path).unwrap()),
        ("day7", "part1") => println!("{}", day7::part1(&path).unwrap()),
        ("day7", "part2") => println!("{}", day7::part2(&path).unwrap()),
        ("day8", "part1") => println!("{}", day8::part1(&path).unwrap()),
        ("day8", "part2") => println!("{}", day8::part2(&path).unwrap()),
        _ => panic!("Bad day/part number. Args should be: day1 part1"),
    }
}
