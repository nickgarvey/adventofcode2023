#[derive(Debug)]
struct Card {
    winning_nums: Vec<usize>,
    player_nums: Vec<usize>,
}

impl Card {
    fn value(&self) -> usize {
        let num_winning = self
            .player_nums
            .iter()
            .filter(|num| self.winning_nums.contains(num))
            .count();
        if num_winning == 0 {
            return 0;
        }

        2_usize.pow((num_winning - 1) as u32)
    }
    fn num_winners(&self) -> usize {
        self.player_nums
            .iter()
            .filter(|num| self.winning_nums.contains(num))
            .count() as usize
    }
}

fn line_to_card(line: &str) -> Card {
    let (_, num_list) = line.split_once(": ").unwrap();
    let (winning_num_str, player_num_str) = num_list.split_once(" | ").unwrap();

    let process_nums = |num_str: &str| {
        num_str
            .split(" ")
            .filter(|num_str| !num_str.is_empty())
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    };

    Card {
        winning_nums: process_nums(winning_num_str),
        player_nums: process_nums(player_num_str),
    }
}

pub fn part1(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let cards: Vec<Card> = input.lines().map(line_to_card).collect();

    cards
        .iter()
        .map(|card| card.value())
        .sum::<usize>()
        .to_string()
}

pub fn part2(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let cards: Vec<Card> = input.lines().map(line_to_card).collect();

    let mut total_count = 0;
    // each element is the count of cards
    let mut active_cards: Vec<usize> = vec![1; cards.len()];

    for idx in 0..cards.len() {
        let card_count = active_cards[idx];
        total_count += card_count;
        let num_winners = cards[idx].num_winners();
        for to_update_idx in (idx + 1)..(idx + 1 + num_winners) {
            active_cards[to_update_idx] += card_count;
        }
    }

    total_count.to_string()
}
