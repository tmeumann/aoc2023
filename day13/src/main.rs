use crate::Reflection::{Horizontal, Vertical};
use ascii::{AsAsciiStr, AsciiChar, AsciiStr, AsciiString};
use std::fs::read_to_string;

#[derive(Copy, Clone)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

fn find_horizontal_reflection(problem: &[&AsciiStr], old: Option<Reflection>) -> Option<usize> {
    (0..problem.len() - 1).find(|&mirror| {
        if let Some(Horizontal(val)) = old {
            if val == mirror {
                return false;
            }
        }
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
}

fn find_vertical_reflection(problem: &[&AsciiStr], old: Option<Reflection>) -> Option<usize> {
    let problem_width = problem[0].len();

    (0..problem_width - 1).find(|&mirror| {
        if let Some(Vertical(val)) = old {
            if val == mirror {
                return false;
            }
        }
        (0..problem_width).all(|offset| {
            if mirror >= offset && mirror + offset + 1 < problem_width {
                let left: Vec<AsciiChar> =
                    problem.iter().map(|line| line[mirror - offset]).collect();
                let right: Vec<AsciiChar> = problem
                    .iter()
                    .map(|line| line[mirror + offset + 1])
                    .collect();

                if left != right {
                    return false;
                }
            }
            true
        })
    })
}

fn find_reflection(problem: &AsciiStr, old: Option<Reflection>) -> Option<Reflection> {
    let vector: Vec<&AsciiStr> = problem.lines().flat_map(|l| l.as_ascii_str()).collect();

    if let Some(val) = find_horizontal_reflection(&vector, old) {
        Some(Horizontal(val))
    } else {
        find_vertical_reflection(&vector, old).map(Vertical)
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let problem_set: Vec<AsciiString> = input
        .split("\n\n")
        .flat_map(AsciiString::from_ascii)
        .collect();

    let result: usize = problem_set
        .iter()
        .map(|problem| {
            if let Some(unadulterated_solution) = find_reflection(&problem, None) {
                for i in 0..problem.len() {
                    let modified_problem = match problem[i] {
                        AsciiChar::Dot => {
                            let mut cloned = problem.clone();
                            cloned[i] = AsciiChar::Hash;
                            cloned
                        }
                        AsciiChar::Hash => {
                            let mut cloned = problem.clone();
                            cloned[i] = AsciiChar::Dot;
                            cloned
                        }
                        AsciiChar::LineFeed => continue,
                        _ => panic!("Oops"),
                    };

                    match find_reflection(&modified_problem, Some(unadulterated_solution)) {
                        Some(Horizontal(val)) => return (val + 1) * 100,
                        Some(Vertical(val)) => return val + 1,
                        _ => {}
                    }
                }
            }

            panic!("Oops")
        })
        .sum();

    println!("{result}");
}
