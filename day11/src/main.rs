use std::fs::read_to_string;

struct Galaxy {
    row: usize,
    column: usize,
}

fn calculate_distance(a: &Galaxy, b: &Galaxy) -> u64 {
    (a.row as i64 - b.row as i64).unsigned_abs()
        + (a.column as i64 - b.column as i64).unsigned_abs()
}

fn sum_shortest_distances(galaxies: &[Galaxy]) -> u64 {
    if let Some((this_galaxy, remaining_galaxies)) = galaxies.split_last() {
        remaining_galaxies
            .iter()
            .map(|other_galaxy| calculate_distance(this_galaxy, other_galaxy))
            .sum::<u64>()
            + sum_shortest_distances(remaining_galaxies)
    } else {
        0
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let num_columns = input.lines().next().unwrap().len();

    let mut universe: Vec<Vec<char>> = input
        .lines()
        .flat_map(|line| -> Vec<Vec<char>> {
            if line.contains('#') {
                vec![line.chars().collect()]
            } else {
                vec![line.chars().collect(), line.chars().collect()]
            }
        })
        .collect();

    for column_index in (0..num_columns).rev() {
        if universe
            .iter()
            .all(|row| row.get(column_index) == Some(&'.'))
        {
            for row in &mut universe {
                row.insert(column_index, '.');
            }
        }
    }

    let galaxies: Vec<Galaxy> = universe
        .iter()
        .enumerate()
        .flat_map(|(row, line)| -> Vec<Galaxy> {
            line.iter()
                .enumerate()
                .filter_map(|(column, c)| {
                    if *c == '#' {
                        Some(Galaxy { row, column })
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    let result = sum_shortest_distances(&galaxies);

    println!("{result}")
}
