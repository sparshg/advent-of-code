use std::collections::HashSet;

use itertools::{iproduct, Itertools};

fn part1(input: &str) -> i32 {
    let input = input.lines().map(|x| x.chars().collect_vec()).collect_vec();
    count_energized(&input, 0, 0, 3)
}

fn count_energized(input: &[Vec<char>], i: usize, j: usize, dir: i32) -> i32 {
    let mut counts = vec![vec![false; input[0].len()]; input.len()];
    let mut set: HashSet<(usize, usize, i32)> = HashSet::new();
    energize(i, j, dir, &mut counts, &mut set, input);
    counts.iter().fold(0, |acc, x| {
        acc + x.iter().fold(0, |a, &y| a + if y { 1 } else { 0 })
    })
}

fn energize(
    mut i: usize,
    mut j: usize,
    mut dir: i32,
    counts: &mut [Vec<bool>],
    set: &mut HashSet<(usize, usize, i32)>,
    input: &[Vec<char>],
) {
    while let Some(c) = input.get(i).and_then(|x| x.get(j)) {
        counts[i][j] = true;
        match c {
            '/' | '\\' => {}
            _ => {
                if !set.insert((i, j, dir % 2)) {
                    break;
                }
            }
        }
        match c {
            '/' => {
                // 0 -> 3, 1 -> 2, 2 -> 1, 3 -> 0
                dir = !dir & 0b11;
            }
            '\\' => {
                // 0 -> 1, 1 -> 0, 2 -> 3, 3 -> 2
                dir = dir & 0b10 | !dir & 0b01
            }
            '|' => {
                if dir % 2 == 1 {
                    energize(i + 1, j, 2, counts, set, input);
                    dir = 0;
                }
            }
            '-' => {
                if dir % 2 == 0 {
                    energize(i, j + 1, 3, counts, set, input);
                    dir = 1;
                }
            }
            _ => {}
        }
        i = (i as i32 + (dir - 1) % 2) as usize;
        j = (j as i32 + (dir - 2) % 2) as usize;
    }
}

fn part2(input: &str) -> i32 {
    let input = input.lines().map(|x| x.chars().collect_vec()).collect_vec();
    let mut res = 0;
    for (i, (j, d)) in iproduct!(0..input.len(), [(0, 3), (input[0].len() - 1, 1)]) {
        res = count_energized(&input, i, j, d).max(res);
    }
    for (j, (i, d)) in iproduct!(0..input.len(), [(0, 2), (input[0].len() - 1, 0)]) {
        res = count_energized(&input, i, j, d).max(res);
    }
    res
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
