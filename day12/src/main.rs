use std::fs::read_to_string;

fn is_valid(condition_record: &[char], groups: &[usize]) -> bool {
    if condition_record.contains(&'?') {
        return false;
    }

    let calculated_groups: Vec<usize> = String::from_iter(condition_record)
        .split_whitespace()
        .map(|group| group.len())
        .collect();

    calculated_groups == groups
}

fn build_possible_records(incomplete_record: Vec<char>) -> Vec<Vec<char>> {
    if let Some((i, _)) = incomplete_record
        .iter()
        .enumerate()
        .find(|(_, &c)| c == '?')
    {
        let mut with_operational = incomplete_record.clone();
        with_operational[i] = ' ';

        let mut with_damaged = incomplete_record.clone();
        with_damaged[i] = '#';

        let mut possibilities = build_possible_records(with_operational);
        possibilities.append(&mut build_possible_records(with_damaged));
        possibilities
    } else {
        vec![incomplete_record]
    }
}

fn count_combinations(condition_record: Vec<char>, groups: &[usize]) -> usize {
    build_possible_records(condition_record)
        .iter()
        .filter(|condition_report| is_valid(condition_report, groups))
        .count()
}

fn main() {
    let input = read_to_string("test.txt").unwrap();

    let result: usize = input
        .lines()
        .map(|l| {
            let sub_strings: Vec<&str> = l.split_whitespace().collect();

            if sub_strings.len() != 2 {
                panic!("Input error");
            }

            let condition_record = sub_strings[0]
                .chars()
                .map(|c| if c == '.' { ' ' } else { c })
                .collect();
            let groups: Vec<usize> = sub_strings[1]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            count_combinations(condition_record, &groups)
        })
        .sum();

    println!("{result}");
}
