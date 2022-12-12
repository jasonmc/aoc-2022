pub fn part_one(input: &str) -> Option<u32> {
    let inter = input.chars().collect::<Vec<char>>();
    let pos = inter.windows(4).position(all_different);
    Some(pos.unwrap() as u32 + 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    let inter = input.chars().collect::<Vec<char>>();
    let pos = inter.windows(14).position(all_different);
    Some(pos.unwrap() as u32 + 14)
}

fn all_different(chars: &[char]) -> bool {
    chars.iter().collect::<std::collections::HashSet<_>>().len() == chars.len()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
