use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(split)
            .map(|instr| do_score(instr.0, instr.1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(split)
            .map(|instr| do_score(instr.0, my_play(instr.0, instr.1)))
            .sum(),
    )
}

fn split(input: &str) -> (char, char) {
    let parts: Vec<&str> = input.split(" ").collect();
    (
        parts[0].chars().next().unwrap(),
        parts[1].chars().next().unwrap(),
    )
}

fn my_play(l: char, need: char) -> char {
    let plays = [
        ('A', 'X', 'Z'),
        ('A', 'Y', 'X'),
        ('A', 'Z', 'Y'),
        ('B', 'X', 'X'),
        ('B', 'Y', 'Y'),
        ('B', 'Z', 'Z'),
        ('C', 'X', 'Y'),
        ('C', 'Y', 'Z'),
        ('C', 'Z', 'X'),
    ];

    let play_map = plays
        .iter()
        .map(|&(l, r, score)| ((l, r), score))
        .collect::<HashMap<(char, char), char>>();

    play_map.get(&(l, need)).cloned().unwrap()
}

fn do_score(l: char, r: char) -> u32 {
    let scores = [
        ('A', 'X', 3),
        ('A', 'Y', 6),
        ('A', 'Z', 0),
        ('B', 'X', 0),
        ('B', 'Y', 3),
        ('B', 'Z', 6),
        ('C', 'X', 6),
        ('C', 'Y', 0),
        ('C', 'Z', 3),
    ];

    let score_map = scores
        .iter()
        .map(|&(l, r, score)| ((l, r), score))
        .collect::<HashMap<(char, char), u32>>();

    score_map.get(&(l, r)).cloned().unwrap()
        + match r {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("bad"),
        }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
