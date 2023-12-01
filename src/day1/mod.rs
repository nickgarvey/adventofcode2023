fn char_to_num(char: Option<u8>) -> u32 {
    match char {
        Some(c) => (c - b'0') as u32,
        None => 0,
    }
}

pub fn part1(input_path: &str) -> String {
    let input = std::fs::read(input_path).unwrap();
    let mut total = 0;
    let mut first: Option<u8> = None;
    let mut last: Option<u8> = None;
    for char in input {
        match char {
            b'0'..=b'9' => {
                if first.is_none() {
                    first = Some(char);
                }
                last = Some(char);
            }
            b'\n' => {
                total += 10 * char_to_num(first) + char_to_num(last);
                first = None;
                last = None;
            }
            _ => continue,
        }
    }
    total += 10 * char_to_num(first) + char_to_num(last);
    total.to_string()
}

pub fn part2(input_path: &str) -> String {
    let input = std::fs::read(input_path).unwrap();
    let mut total: u32 = 0;
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    let words_of_interest: Vec<&[u8]> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .map(|word| word.as_bytes())
    .collect();

    for (pos, char) in input.iter().enumerate() {
        match char {
            b'0'..=b'9' => {
                if first.is_none() {
                    first = Some((char - b'0').into());
                }
                last = Some((char - b'0').into());
            }
            b'\n' => {
                total += 10 * first.unwrap_or_default() + last.unwrap_or_default();
                first = None;
                last = None;
            }
            _ => {
                for (idx, word) in words_of_interest.iter().enumerate() {
                    if input[pos..].starts_with(word) {
                        if first.is_none() {
                            first = Some(idx as u32 + 1);
                        }
                        last = Some(idx as u32 + 1);
                        break;
                    }
                }
            }
        }
    }

    total += 10 * first.unwrap_or_default() + last.unwrap_or_default();
    total.to_string()
}
