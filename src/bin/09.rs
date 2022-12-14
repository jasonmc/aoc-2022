use itertools::Itertools;
use std::io::{self, Write};
use std::iter;

pub fn part_one(input: &str) -> Option<u32> {
    let instrs = input
        .lines()
        .map(|l| {
            let (dir, count) = l.split_once(' ').unwrap();
            let dir = dir.chars().next().unwrap();
            let count = count.parse::<u32>().unwrap();
            (dir, count)
        })
        .collect::<Vec<_>>();

    let unique_tail_pos = instrs
        .iter()
        .flat_map(|(direction, count)| iter::repeat(*direction).take(*count as usize))
        .scan(((0, 0), (0, 0)), |acc, dir| {
            let (h, t) = acc;
            *acc = calculate_pos(*h, *t, dir);
            Some(*acc)
        })
        .map(|(_, t)| t)
        .unique()
        .count();

    Some(unique_tail_pos as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instrs = input
        .lines()
        .map(|l| {
            let (dir, count) = l.split_once(' ').unwrap();
            let dir = dir.chars().next().unwrap();
            let count = count.parse::<u32>().unwrap();
            (dir, count)
        })
        .collect::<Vec<_>>();

    let init = iter::repeat((0, 0)).take(10).collect::<Vec<_>>();

    let unique_tail_pos = instrs
        .iter()
        .flat_map(|(direction, count)| iter::repeat(*direction).take(*count as usize))
        .scan(init, |acc, dir| {
            *acc = calculate_n_pos(acc.clone(), dir);
            Some(acc.last().unwrap().clone())
        })
        .unique()
        .count();

    Some(unique_tail_pos as u32) //4899 is too high
}

fn calculate_pos(
    head_curr: (i32, i32),
    tail_curr: (i32, i32),
    direction: char,
) -> ((i32, i32), (i32, i32)) {
    let (hx, hy) = head_curr;
    let new_head = match direction {
        'R' => (hx + 1, hy),
        'L' => (hx - 1, hy),
        'U' => (hx, hy + 1),
        'D' => (hx, hy - 1),
        _ => panic!("bad"),
    };
    let new_tail = if is_adjacent(new_head, tail_curr) {
        tail_curr
    } else {
        head_curr
    };

    (new_head, new_tail)
}

fn calculate_n_pos(parts: Vec<(i32, i32)>, direction: char) -> Vec<(i32, i32)> {
    let head: (i32, i32) = parts.first().unwrap().clone();
    let tail: &[(i32, i32)] = &parts[1..];

    let (hx, hy) = head;
    let new_head = match direction {
        'R' => (hx + 1, hy),
        'L' => (hx - 1, hy),
        'U' => (hx, hy + 1),
        'D' => (hx, hy - 1),
        _ => panic!("bad"),
    };

    let tails = tail.iter().scan(new_head, |acc, pos| {
        let new_pos = rule(*acc, *pos);
        *acc = new_pos;
        Some(new_pos)
    });

    iter::once(new_head).chain(tails).collect::<Vec<_>>()
}

fn rule(left_new: (i32, i32), curr: (i32, i32)) -> (i32, i32) {
    //curr (4, 0) becoming (5, 1) to follow (5, 2)
    //curr (3, 0) becoming (4, 0) to follow (5, 1)
    //
    // if (2,1) or (1,2) then move (1,1)
    // e.g. delta (2,1) -> (1,1)
    // if (2,0) then (1,0)
    // if (0,2) then (0,1)

    if is_adjacent(left_new, curr) {
        curr
    } else {
        let (cx, cy) = curr;
        let (lnx, lny) = left_new;
        let dx = lnx - cx;
        let dy = lny - cy;
        if (dx.abs() > 1 && dy.abs() > 0) || (dy.abs() > 1 && dx.abs() > 0) {
            (cx + clamp_signed(dx), cy + clamp_signed(dy))
        } else {
            if dx.abs() > 1 {
                (cx + clamp_signed(dx), cy)
            } else {
                assert!(dy.abs() > 1);
                (cx, cy + clamp_signed(dy))
            }
        }
    }
}

fn clamp_signed(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n > 0 {
        1
    } else {
        -1
    }
}

fn is_adjacent((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> bool {
    (x1 - x2).abs() <= 1 && (y1 - y2).abs() <= 1
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
