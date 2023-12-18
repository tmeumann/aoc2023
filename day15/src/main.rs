use ascii::{AsAsciiStr, AsciiChar, AsciiStr, AsciiString, IntoAsciiString};
use std::collections::VecDeque;
use std::fs::read_to_string;

struct Lens {
    label: AsciiString,
    focal_length: usize,
}

impl Lens {
    fn new(label: AsciiString, focal_length: usize) -> Self {
        Self {
            label,
            focal_length,
        }
    }
}

struct LensBox {
    id: usize,
    lenses: VecDeque<Lens>,
}

impl LensBox {
    fn new(id: usize) -> Self {
        Self {
            id,
            lenses: VecDeque::new(),
        }
    }

    fn insert_lens(&mut self, label: &AsciiStr, focal_length: usize) {
        if let Some(lens) = self.lenses.iter_mut().find(|lens| lens.label == label) {
            lens.focal_length = focal_length;
        } else {
            self.lenses
                .push_back(Lens::new(label.to_ascii_string(), focal_length));
        }
    }

    fn remove_lens(&mut self, label: &AsciiStr) {
        for i in 0..self.lenses.len() {
            if self.lenses[i].label == label {
                self.lenses.remove(i);
                break;
            }
        }
    }

    fn calculate_focusing_power(&self) -> usize {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, Lens { focal_length, .. })| (1 + self.id) * (i + 1) * focal_length)
            .sum()
    }
}

fn hash(s: &AsciiStr) -> usize {
    s.chars().fold(0, |acc, character| {
        (acc + character.as_byte() as usize) * 17 % 256
    })
}

fn main() {
    let input = read_to_string("input.txt")
        .unwrap()
        .into_ascii_string()
        .unwrap();

    let mut boxes: Vec<LensBox> = (0..256).map(LensBox::new).collect();

    for step in input.split(AsciiChar::Comma) {
        match step.last() {
            Some(AsciiChar::Minus) => {
                let label = step.slice_ascii(0..step.len() - 1).unwrap();
                let hash = hash(label);
                boxes[hash].remove_lens(label);
            }
            Some(character) => {
                let label = step.slice_ascii(0..step.len() - 2).unwrap();
                let hash = hash(label);
                let focal_length = character.to_string().parse::<usize>().unwrap();
                boxes[hash].insert_lens(label, focal_length);
            }
            _ => panic!("Unexpected empty string!"),
        }
    }

    let result: usize = boxes.iter().map(|b| b.calculate_focusing_power()).sum();

    println!("{result}")
}
