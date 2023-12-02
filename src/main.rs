use clap::Parser;

mod day1;
mod day2;

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
        _ => panic!("Bad day/part number. Args should be: day1 part1"),
    }
}
