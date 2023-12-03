#[derive(Debug)]
struct MatchedNum {
    num: u32,
    x_start: usize,
    y_start: usize,
    len: usize,
}

struct Gear {
    x_pos: usize,
    y_pos: usize,
}

fn parse_nums(
    input: &Vec<u8>,
    nums: &mut Vec<MatchedNum>,
    gears: &mut Vec<Gear>,
    line_len: &mut usize,
    y_end: &mut usize,
) {
    let mut in_progress_num: Vec<u8> = Vec::new();
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut saw_newline = false;

    for char in input.iter() {
        if saw_newline {
            x_pos = 0;
            y_pos += 1;
            saw_newline = false;
        }

        match char {
            b'0'..=b'9' => {
                in_progress_num.push(*char);
            }
            c => {
                if *c == b'\n' {
                    saw_newline = true;
                    *line_len = x_pos;
                } else if *c == b'*' {
                    gears.push(Gear { x_pos, y_pos });
                }
                if !in_progress_num.is_empty() {
                    let val = String::from_utf8_lossy(in_progress_num.as_slice())
                        .parse::<u32>()
                        .unwrap();
                    nums.push(MatchedNum {
                        num: val,
                        x_start: x_pos - in_progress_num.len(),
                        y_start: y_pos,
                        len: in_progress_num.len(),
                    });
                    in_progress_num.clear();
                }
            }
        }
        x_pos += 1;
    }
    *y_end = y_pos;
}

pub fn part1(input_path: &str) -> String {
    let input = std::fs::read(input_path).unwrap();
    let mut nums: Vec<MatchedNum> = Vec::new();
    let mut gears: Vec<Gear> = Vec::new();
    let mut line_len = 0;
    let mut y_end = 0;
    parse_nums(&input, &mut nums, &mut gears, &mut line_len, &mut y_end);

    let mut total = 0;
    for num in nums.iter() {
        let x_scan_start = if num.x_start == 0 { 0 } else { num.x_start - 1 };
        let x_scan_end = if num.x_start + num.len == line_len {
            line_len
        } else {
            num.x_start + num.len
        };

        let y_scan_start = if num.y_start == 0 { 0 } else { num.y_start - 1 };
        let y_scan_end = if num.y_start == y_end {
            y_end
        } else {
            num.y_start + 1
        };

        'outer: for y in y_scan_start..=y_scan_end {
            for x in x_scan_start..=x_scan_end {
                let to_check_idx = y * (line_len + 1) + x;
                match input[to_check_idx] as char {
                    '0'..='9' => continue,
                    '.' => continue,
                    '\n' => continue,
                    _ => {
                        total += num.num;
                        break 'outer;
                    }
                }
            }
        }
    }

    total.to_string()
}

pub fn part2(input_path: &str) -> String {
    let input = std::fs::read(input_path).unwrap();
    let mut nums: Vec<MatchedNum> = Vec::new();
    let mut gears: Vec<Gear> = Vec::new();
    let mut line_len = 0;
    let mut y_end = 0;
    parse_nums(&input, &mut nums, &mut gears, &mut line_len, &mut y_end);
    let mut total = 0;

    for gear in gears.iter() {
        let mut nearby_nums = Vec::new();

        for num in nums.iter() {
            let x_scan_start = if num.x_start == 0 { 0 } else { num.x_start - 1 };
            let x_scan_end = if num.x_start + num.len == line_len {
                line_len
            } else {
                num.x_start + num.len
            };

            let y_scan_start = if num.y_start == 0 { 0 } else { num.y_start - 1 };
            let y_scan_end = if num.y_start == y_end {
                y_end
            } else {
                num.y_start + 1
            };

            'outer: for y in y_scan_start..=y_scan_end {
                for x in x_scan_start..=x_scan_end {
                    if x == gear.x_pos && y == gear.y_pos {
                        nearby_nums.push(num);
                        break 'outer;
                    }
                }
            }
        }

        if nearby_nums.len() == 2 {
            total += nearby_nums[0].num * nearby_nums[1].num;
        }
    }

    total.to_string()
}
