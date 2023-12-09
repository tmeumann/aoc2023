use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Hand {
    FiveOfAKind(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    TwoPair(Vec<Card>),
    OnePair(Vec<Card>),
    HighCard(Vec<Card>),
}

impl TryFrom<&str> for Hand {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cards: Vec<Card> = value
            .chars()
            .map(Card::try_from)
            .collect::<Result<_, ()>>()?;

        let counter: HashMap<&Card, u64> =
            cards
                .iter()
                .fold(HashMap::with_capacity(5), |mut counter, card| {
                    counter.entry(card).and_modify(|n| *n += 1).or_insert(1);
                    counter
                });

        let mut counts: Vec<u64> = counter.into_values().collect();

        counts.sort();

        let highest_count = counts.pop();
        let second_highest_count = counts.pop();

        match highest_count {
            Some(5) => Ok(Self::FiveOfAKind(cards)),
            Some(4) => Ok(Self::FourOfAKind(cards)),
            Some(3) => match second_highest_count {
                Some(2) => Ok(Self::FullHouse(cards)),
                Some(1) => Ok(Self::ThreeOfAKind(cards)),
                _ => Err(()),
            },
            Some(2) => match second_highest_count {
                Some(2) => Ok(Self::TwoPair(cards)),
                Some(1) => Ok(Self::OnePair(cards)),
                _ => Err(()),
            },
            Some(1) => Ok(Self::HighCard(cards)),
            _ => Err(()),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err(()),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Play {
    hand: Hand,
    pub bid: u64,
}

impl TryFrom<&str> for Play {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let sub_strings: Vec<&str> = value.split_whitespace().collect();

        let hand: Hand = Hand::try_from(*sub_strings.first().ok_or(())?)?;
        let bid: u64 = sub_strings.last().ok_or(())?.parse().or(Err(()))?;

        Ok(Play { hand, bid })
    }
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open input file.");
    let reader = BufReader::new(file);

    let mut plays: Vec<Play> = reader
        .lines()
        .map_while(|l| l.ok())
        .map(|s| Play::try_from(s.as_str()))
        .collect::<Result<_, ()>>()
        .expect("Failed to parse input.");

    plays.sort();

    let result: u64 = plays
        .iter()
        .rev()
        .enumerate()
        .map(|(i, Play { bid, .. })| (i + 1) as u64 * bid)
        .sum();

    println!("{result}");
}
