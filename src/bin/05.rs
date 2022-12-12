use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, newline, satisfy, space1},
    combinator::map_res,
    multi::{count, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};

pub fn part_one(input: &str) -> Option<String> {
    let (stacks, instrs) = load(input);
    let processed = instrs.iter().cloned().fold(stacks, execute_instr);
    Some(get_top_of_stacks(processed))
}

pub fn part_two(input: &str) -> Option<String> {
    let (stacks, instrs) = load(input);
    let processed = instrs.iter().cloned().fold(stacks, execute_instr_v2);
    Some(get_top_of_stacks(processed))
}

fn load(input: &str) -> (Vec<Vec<char>>, Vec<(u32, u32, u32)>) {
    let (_, (stacks, instrs)) = parser(input).unwrap();

    assert!(!stacks.is_empty());

    let transformed = (0..stacks[0].len())
        .map(|i| {
            stacks
                .iter()
                .rev()
                .map(|inner| inner[i].clone())
                .take_while(|y| y.is_some())
                .map(|y| y.unwrap())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    (transformed, instrs)
}

fn get_top_of_stacks(stacks: Vec<Vec<char>>) -> String {
    stacks.iter().map(|x| x.last().unwrap()).collect::<String>()
}

fn execute_instr_v2(v: Vec<Vec<char>>, (num, from, to): (u32, u32, u32)) -> Vec<Vec<char>> {
    let mut v = v;
    let from = from - 1;
    let to = to - 1;
    let from_len = v[from as usize].len();
    let vals = v[from as usize].split_off(from_len - num as usize);
    v[to as usize].extend_from_slice(&vals);
    v
}

fn execute_instr(v: Vec<Vec<char>>, (num, from, to): (u32, u32, u32)) -> Vec<Vec<char>> {
    (0..num).fold(v, |acc, _| move_crate(acc, from - 1, to - 1))
}

fn move_crate(v: Vec<Vec<char>>, from: u32, to: u32) -> Vec<Vec<char>> {
    let mut v = v;
    let val = v[from as usize].pop().unwrap();
    v[to as usize].push(val);
    v
}

fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    map_res(
        delimited(char('['), satisfy(|c: char| c.is_uppercase()), char(']')),
        |c: char| Ok::<Option<char>, &str>(Some(c)),
    )(input)
}

fn parse_no_crate(input: &str) -> IResult<&str, Option<char>> {
    map_res(count(char(' '), 3), |_: Vec<char>| {
        Ok::<Option<char>, &str>(None)
    })(input)
}

fn crate_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(char(' '), alt((parse_crate, parse_no_crate)))(input)
}

fn number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn line_of_nums(input: &str) -> IResult<&str, (char, Vec<u32>, char)> {
    tuple((char(' '), separated_list1(space1, number), char(' ')))(input)
}

fn instr_line(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (i, (_, b, _, d, _, f)) = tuple((
        tag("move "),
        number,
        tag(" from "),
        number,
        tag(" to "),
        number,
    ))(input)?;
    Ok((i, (b, d, f)))
}

fn parser(input: &str) -> IResult<&str, (Vec<Vec<Option<char>>>, Vec<(u32, u32, u32)>)> {
    let (i, crates) = separated_list1(newline, crate_line)(input)?;
    let (i, _) = tuple((newline, line_of_nums, newline, newline))(i)?;
    let (i, instrs) = separated_list1(newline, instr_line)(i)?;
    Ok((i, (crates, instrs)))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".into()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".into()));
    }
}
