use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
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
    let expansion: usize = 1_000_000;

    let num_columns = input.lines().next().unwrap().len();

    let mut universe: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut empty_rows: HashSet<usize> = HashSet::new();
    for (row_index, row) in universe.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            empty_rows.insert(row_index);
        }
    }

    let mut empty_columns: HashSet<usize> = HashSet::new();
    for column_index in 0..num_columns {
        if universe
            .iter()
            .all(|row| row.get(column_index) == Some(&'.'))
        {
            empty_columns.insert(column_index);
        }
    }

    let mut row_count = 0;
    let galaxies: Vec<Galaxy> = universe
        .iter()
        .enumerate()
        .flat_map(|(row_index, line)| -> Vec<Galaxy> {
            let mut column_count = 0;
            let galaxies = line
                .iter()
                .enumerate()
                .filter_map(|(column_index, c)| {
                    let galaxy = if *c == '#' {
                        Some(Galaxy {
                            row: row_count,
                            column: column_count,
                        })
                    } else {
                        None
                    };

                    column_count += if empty_columns.contains(&column_index) {
                        expansion
                    } else {
                        1
                    };

                    galaxy
                })
                .collect();

            row_count += if empty_rows.contains(&row_index) {
                expansion
            } else {
                1
            };

            galaxies
        })
        .collect();

    let result = sum_shortest_distances(&galaxies);

    println!("{result}")
}
