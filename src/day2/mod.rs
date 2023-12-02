use core::panic;

pub fn part1(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();

    const RED_MAX: u32 = 12;
    const GREEN_MAX: u32 = 13;
    const BLUE_MAX: u32 = 14;

    let mut total = 0;
    for line in input.lines() {
        let trim_game = line.split_at(5).1;

        let colon_idx = trim_game.find(":").unwrap();

        let split = trim_game.split_at(colon_idx);
        let game_id: u32 = split.0.parse().unwrap();

        let mut passed = true;
        for cube_set in split.1[2..].split("; ") {
            for cube_count in cube_set.split(", ") {
                let cube_split = cube_count.split_once(" ").unwrap();
                let count = cube_split.0.parse::<u32>().unwrap();
                let color = cube_split.1;

                match color {
                    "red" => {
                        passed &= count <= RED_MAX;
                    }
                    "green" => {
                        passed &= count <= GREEN_MAX;
                    }
                    "blue" => {
                        passed &= count <= BLUE_MAX;
                    }
                    _ => panic!("Unknown color: {}", color),
                }
            }
        }
        if passed {
            total += game_id;
        }
    }
    total.to_string()
}

pub fn part2(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();

    let mut total = 0;
    // For each game
    for line in input.lines() {
        // Trim "Game "
        let trim_game = line.split_at(5).1;
        let colon_idx = trim_game.find(":").unwrap();
        let split = trim_game.split_at(colon_idx);

        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;
        // For each set of cubes
        for cube_set in split.1[2..].split("; ") {
            // For each count of a single cube
            for cube_count in cube_set.split(", ") {
                let cube_split = cube_count.split_once(" ").unwrap();
                let count = cube_split.0.parse::<u32>().unwrap();
                let color = cube_split.1;

                match color {
                    "red" => {
                        red_max = red_max.max(count);
                    }
                    "green" => {
                        green_max = green_max.max(count);
                    }
                    "blue" => {
                        blue_max = blue_max.max(count);
                    }
                    _ => panic!("Unknown color: {}", color),
                }
            }
        }
        total += red_max * green_max * blue_max;
    }
    total.to_string()
}
