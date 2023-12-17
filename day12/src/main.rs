use ascii::{AsAsciiStr, AsciiChar, AsciiStr, AsciiString};
use rayon::prelude::*;
use std::fs::read_to_string;

// called when the last character was a '#' or equivalent
fn consume_group(chars: &[AsciiChar], group_size: usize, remaining_groups: &[usize]) -> usize {
    if group_size == 0 {
        match chars.split_first() {
            Some((AsciiChar::Dot | AsciiChar::Question, remaining_chars)) => {
                // dot or question mark -- we're done consuming '#' for now
                consume_space(remaining_chars, remaining_groups)
            }
            Some((AsciiChar::Hash, _)) => {
                // we've hit a hash, but run out of group. no dice
                0
            }
            None => {
                if remaining_groups.is_empty() {
                    // group has been consumed, no more input and no more groups
                    1
                } else {
                    // group has been consumed, no more input but we have more groups!
                    0
                }
            }
            Some(_) => panic!("Unrecognised character"),
        }
    } else {
        match chars.split_first() {
            Some((AsciiChar::Hash | AsciiChar::Question, remaining_chars)) => {
                consume_group(remaining_chars, group_size - 1, remaining_groups)
            }
            Some((AsciiChar::Dot, _)) | None => {
                // expected more, but group ended
                0
            }
            Some(_) => panic!("Unrecognised character"),
        }
    }
}

fn consume_space(chars: &[AsciiChar], groups: &[usize]) -> usize {
    match groups.split_first() {
        // short circuit if we have no groups
        None => {
            if chars.iter().any(|&character| character == AsciiChar::Hash) {
                // no more groups, but there's a '#' somewhere
                0
            } else {
                // no more groups, no more hashes (all remaining question marks must be '.')
                1
            }
        }
        Some((group_size, remaining_groups)) => {
            match chars.split_first() {
                Some((AsciiChar::Dot, remaining_chars)) => consume_space(remaining_chars, groups),
                Some((AsciiChar::Hash, remaining_chars)) => {
                    consume_group(remaining_chars, *group_size - 1, remaining_groups)
                }
                Some((AsciiChar::Question, remaining_chars)) => {
                    consume_group(remaining_chars, *group_size - 1, remaining_groups)
                        + consume_space(remaining_chars, groups)
                }
                None => 0, // we expect a group, but we've run over the end of the string
                Some(_) => panic!("Unrecognised character"),
            }
        }
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let result: usize = input
        .par_lines()
        .map(|line| {
            let sub_strings: Vec<&str> = line.split_whitespace().collect();

            if sub_strings.len() != 2 {
                panic!("Input error");
            }

            let folded_record: &AsciiStr = sub_strings[0].as_ascii_str().unwrap();

            let mut unfolded_record: AsciiString =
                AsciiString::with_capacity(folded_record.len() * 5 + 4);
            unfolded_record.extend(folded_record);
            for _ in 0..4 {
                unfolded_record.push(AsciiChar::Question);
                unfolded_record.extend(folded_record);
            }

            let folded_groups: Vec<usize> = sub_strings[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let mut unfolded_groups: Vec<usize> = Vec::with_capacity(folded_groups.len() * 5);
            for _ in 0..5 {
                unfolded_groups.extend(&folded_groups);
            }

            let possible_combinations = consume_space(unfolded_record.as_slice(), &unfolded_groups);

            println!("{line}: {possible_combinations}");

            possible_combinations
        })
        .sum();

    println!("{result}");
}
