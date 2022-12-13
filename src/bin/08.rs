pub fn part_one(input: &str) -> Option<u32> {
    let vals = parse_input(input);

    let transposed = transpose(vals.clone());

    let x: usize = vals
        .iter()
        .enumerate()
        .map(|(row_num, row)| {
            row.iter()
                .enumerate()
                .filter(|(column_num, val)| {
                    path_to_either_side(**val, *column_num, row.clone())
                        || path_to_either_side(**val, row_num, transposed[*column_num].clone())
                })
                .count()
        })
        .sum();

    Some(x as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let vals = parse_input(input);

    let transposed = transpose(vals.clone());

    let x = vals
        .iter()
        .enumerate()
        .flat_map(|(row_num, row)| -> Vec<usize> {
            row.iter()
                .enumerate()
                .map(|(column_num, val)| -> usize {
                    trees_visible(*val, column_num, row.clone())
                        * trees_visible(*val, row_num, transposed[column_num].clone())
                })
                .collect()
        })
        .max()
        .unwrap();

    Some(x as u32) // 300 is too low, 384 is too low
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>()
}

fn path_to_either_side(val: u32, position: usize, row: Vec<u32>) -> bool {
    let left = row[..position].iter().all(|j| *j < val);
    let right = row[position + 1..].iter().all(|j| *j < val);
    left || right
}

fn trees_visible(val: u32, position: usize, row: Vec<u32>) -> usize {
    let right = look(&row[position + 1..], val);
    let left = look(
        &row[..position].iter().rev().cloned().collect::<Vec<u32>>(),
        val,
    );
    left * right
}

fn look(items: &[u32], val: u32) -> usize {
    items
        .iter()
        .scan(false, |state, &x| {
            if *state {
                None
            } else {
                if x >= val {
                    *state = true;
                }
                Some(x)
            }
        })
        .count()
}

fn transpose(v: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<u32>>())
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
