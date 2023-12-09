use std::cell::Cell;
use std::collections::HashMap;
use std::fs::read_to_string;
use typed_arena::Arena;

enum Direction {
    Right,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Self::Right),
            'L' => Ok(Self::Left),
            _ => Err(()),
        }
    }
}

struct Node<'a> {
    name: String,
    left: Cell<Option<&'a Node<'a>>>,
    right: Cell<Option<&'a Node<'a>>>,
}

impl<'a> Node<'a> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            left: Cell::new(None),
            right: Cell::new(None),
        }
    }

    pub fn get_neighbour(&'a self, direction: &Direction) -> &'a Node {
        match direction {
            Direction::Right => self.right.get().expect("A node without a neighbour T_T"),
            Direction::Left => self.left.get().expect("A node without a neighbour T_T"),
        }
    }
}

fn parse_node_spec(input: &str) -> Result<(&str, &str, &str), ()> {
    let name = input.split_whitespace().next().ok_or(())?;

    let neighbours: Vec<&str> = input
        .split('=')
        .last()
        .ok_or(())?
        .trim()
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split_whitespace()
        .map(|s| s.trim_matches(','))
        .collect();

    if neighbours.len() != 2 {
        return Err(());
    }

    let left = neighbours.first().ok_or(())?;
    let right = neighbours.last().ok_or(())?;

    Ok((name, left, right))
}

fn build_map<'a>(input: &str, arena: &'a Arena<Node<'a>>) -> HashMap<String, &'a Node<'a>> {
    let nodes: HashMap<String, &Node> = input
        .lines()
        .skip(2)
        .flat_map(parse_node_spec)
        .map(|(name, _, _)| (name.into(), &*arena.alloc(Node::new(name.into()))))
        .collect();

    for line in input.lines().skip(2) {
        let (name, left, right) = parse_node_spec(line).expect("Failed to parse node name");

        let node = nodes.get(name).expect("Failed to get node from hashmap");

        let left = nodes.get(left).expect("Failed to get node from hashmap");

        let right = nodes.get(right).expect("Failed to get node from hashmap");

        node.left.set(Some(left));
        node.right.set(Some(right));
    }

    nodes
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input file");

    let directions: Vec<Direction> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(Direction::try_from)
        .collect::<Result<_, _>>()
        .expect("Failed to parse directions");

    let node_arena = Arena::new();

    let nodes = build_map(&input, &node_arena);

    let mut current_node: &Node = nodes.get("AAA").expect("No starting node!");

    let result = directions
        .iter()
        .cycle()
        .map_while(|direction| {
            if current_node.name == "ZZZ" {
                None
            } else {
                current_node = current_node.get_neighbour(direction);
                Some(())
            }
        })
        .count();

    println!("{result}");
}
