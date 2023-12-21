use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn part1(input: &str) -> i32 {
    let input = input.lines().map(|x| x.chars().collect_vec()).collect_vec();
    let mut queue = VecDeque::new();
    for (i, row) in input.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == 'S') {
            queue.push_back((i, j));
            break;
        }
    }
    for _ in 0..64 {
        let n = queue.len();
        let mut set = HashSet::new();
        for _ in 0..n {
            let (i, j) = queue.pop_front().unwrap();
            for d in 0..4 {
                let (newi, newj) = (
                    (i as i32 + (d - 1) % 2) as usize,
                    (j as i32 + (d - 2) % 2) as usize,
                );
                if newi < input.len() && newj < input[0].len() && input[newi][newj] != '#' {
                    if set.insert((newi, newj)) {
                        queue.push_back((newi, newj));
                    }
                }
            }
        }
    }
    queue.len() as i32
}

fn part2(input: &str) -> i32 {
    0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    return Some(part1(input));
    // return Some(part2(input));
    None
}
