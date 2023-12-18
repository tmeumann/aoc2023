use ascii::{AsAsciiStr, AsciiChar, AsciiStr};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let problem_set: Vec<Vec<&AsciiStr>> = input
        .split("\n\n")
        .map(|s| s.lines().flat_map(|l| l.as_ascii_str()).collect())
        .collect();

    let row_count: usize = problem_set
        .iter()
        .filter_map(|problem| {
            (0..problem.len() - 1)
                .find(|&mirror| {
                    (0..problem.len()).all(|offset| {
                        if mirror >= offset {
                            if let Some(top) = problem.get(mirror - offset) {
                                if let Some(bottom) = problem.get(mirror + offset + 1) {
                                    if top != bottom {
                                        return false;
                                    }
                                }
                            }
                        }
                        true
                    })
                })
                .map(|v| (v + 1) * 100)
        })
        .sum();

    let column_count: usize = problem_set
        .iter()
        .filter_map(|problem| {
            let problem_width = problem[0].len();
            (0..problem_width - 1)
                .find(|&mirror| {
                    (0..problem_width).all(|offset| {
                        if mirror >= offset && mirror + offset + 1 < problem_width {
                            let left: Vec<AsciiChar> =
                                problem.iter().map(|&line| line[mirror - offset]).collect();
                            let right: Vec<AsciiChar> = problem
                                .iter()
                                .map(|&line| line[mirror + offset + 1])
                                .collect();

                            if left != right {
                                return false;
                            }
                        }
                        true
                    })
                })
                .map(|v| v + 1)
        })
        .sum();

    let result = row_count + column_count;

    println!("{result}");
}
