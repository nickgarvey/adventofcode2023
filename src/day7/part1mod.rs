use nom::{
    bytes::complete::{tag, take},
    character::complete::{multispace1, u64},
    combinator::{all_consuming, opt},
    multi::separated_list0,
    sequence::tuple,
    Finish, IResult,
};
use std::{cmp::Ordering, collections::HashMap};
use std::{fmt::Display, fs};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: [u8; 5],
    bid: u64,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut bins = HashMap::new();
        for card in self.cards {
            bins.insert(card, bins.get(&card).unwrap_or(&0) + 1);
        }
        if bins.len() == 1 {
            HandType::FiveOfAKind
        } else if bins.len() == 2 {
            if bins.values().any(|v| *v == 4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        } else if bins.len() == 3 {
            if bins.values().any(|v| *v == 3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        } else if bins.len() == 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

fn byte_to_val(byte: u8) -> u8 {
    match byte {
        b'2'..=b'9' => byte - b'0',
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => panic!("Invalid card value"),
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let my_hand_type = self.hand_type();
        let other_hand_type = other.hand_type();
        if my_hand_type == other_hand_type {
            for i in 0..5 {
                if self.cards[i] != other.cards[i] {
                    return Some(byte_to_val(self.cards[i]).cmp(&byte_to_val(other.cards[i])));
                }
            }
            panic!("Two hands are equal")
        } else {
            Some(my_hand_type.cmp(&other_hand_type))
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let card_str: String = self.cards.iter().map(|c| *c as char).collect();
        write!(f, "{} {}", card_str, self.bid)
    }
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (remain, (card_str, _, bid)) = tuple((take(5u32), tag(" "), u64))(input)?;
    let mut cards = [0u8; 5];
    cards
        .iter_mut()
        .zip(card_str.chars())
        .for_each(|(c, ch)| *c = ch as u8);
    Ok((remain, Hand { cards, bid }))
}

fn parse(input: &str) -> Result<Vec<Hand>, nom::error::Error<&str>> {
    let (_, (hands, _)) = all_consuming(tuple((
        separated_list0(tag("\n"), parse_hand),
        opt(multispace1),
    )))(input)
    .finish()?;
    Ok(hands)
}

pub fn part1(input_path: &str) -> Result<u64, String> {
    let input = fs::read_to_string(input_path).unwrap();
    let mut hands = parse(&input).map_err(|e| format!("Parse Error: {}", e))?;
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
    Ok(hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + h.bid * (i + 1) as u64))
}
