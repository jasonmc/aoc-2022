use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(split)
            .map(|t| {
                let a: HashSet<char> = t.0.chars().collect();
                let b: HashSet<char> = t.1.chars().collect();
                let c = a.intersection(&b).cloned().collect::<Vec<char>>();
                calculate_priority(c.first().cloned().unwrap())
            })
            .sum(),
    )
}

fn split(s: &str) -> (&str, &str) {
    s.split_at(s.len() / 2)
}

fn calculate_priority(c: char) -> u32 {
    match c {
        'A'..='Z' => c as u32 - 38,
        'a'..='z' => c as u32 - 96,
        _ => panic!("bad"),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .tuples::<(_, _, _)>()
            .map(find_common)
            .map(calculate_priority)
            .sum::<u32>(),
    )
}

fn find_common(dudes: (&str, &str, &str)) -> char {
    let b: HashSet<char> = dudes.1.chars().collect();
    let c: HashSet<char> = dudes.2.chars().collect();
    dudes
        .0
        .chars()
        .filter(|x| b.contains(x) && c.contains(x))
        .unique()
        .exactly_one()
        .unwrap()
        .clone()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
