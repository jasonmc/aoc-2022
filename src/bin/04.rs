use nom::{
    character::complete::{char, digit1},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};

pub fn part_one(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .map(|l| parse_line(l))
        .filter(|&x| contains(x))
        .count();
    Some(u32::try_from(count).unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .map(|l| parse_line(l))
        .filter(|&x| does_overlap(x))
        .count();
    Some(u32::try_from(count).unwrap())
}

fn contains(((al, au), (bl, bu)): ((u32, u32), (u32, u32))) -> bool {
    (au <= bu && al >= bl) || (bu <= au && bl >= al)
}

fn does_overlap(((al, au), (bl, bu)): ((u32, u32), (u32, u32))) -> bool {
    al <= bu && bl <= au
}

fn number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn range(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(number, char('-'), number)(input)
}

fn ranges(input: &str) -> IResult<&str, ((u32, u32), (u32, u32))> {
    separated_pair(range, char(','), range)(input)
}

fn parse_line(input: &str) -> ((u32, u32), (u32, u32)) {
    let (_, r) = ranges(input).unwrap();
    r
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
