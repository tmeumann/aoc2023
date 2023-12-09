use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Hand {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPair(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FiveOfAKind(Vec<Card>),
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

        let mut counts: Vec<(u64, &Card)> = counter
            .iter()
            .map(|(card, count)| (*count, *card))
            .collect();

        counts.sort();

        let (mut highest_count, most_numerous_card) = counts.pop().ok_or(())?;

        let mut second_highest = counts.pop().map(|(n, _)| n);

        if *most_numerous_card == Card::Joker {
            highest_count += second_highest.unwrap_or(0);
            second_highest = counts.pop().map(|(n, _)| n)
        } else {
            let number_of_jokers = counter.get(&Card::Joker).unwrap_or(&0);
            highest_count += number_of_jokers
        }

        match highest_count {
            5 => Ok(Self::FiveOfAKind(cards)),
            4 => Ok(Self::FourOfAKind(cards)),
            3 => match second_highest {
                Some(2) => Ok(Self::FullHouse(cards)),
                Some(1) => Ok(Self::ThreeOfAKind(cards)),
                _ => Err(()),
            },
            2 => match second_highest {
                Some(2) => Ok(Self::TwoPair(cards)),
                Some(1) => Ok(Self::OnePair(cards)),
                _ => Err(()),
            },
            1 => Ok(Self::HighCard(cards)),
            _ => Err(()),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            'J' => Ok(Self::Joker),
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
        .enumerate()
        .map(|(i, Play { bid, .. })| (i + 1) as u64 * bid)
        .sum();

    println!("{result}");
}
