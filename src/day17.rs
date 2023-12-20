use std::{
    cmp::Reverse,
    collections::{binary_heap::BinaryHeap, HashSet},
};

use itertools::Itertools;

fn solve(input: &str, part2: bool) -> i32 {
    let input = input
        .lines()
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let mut vis = HashSet::new();
    let mut heap = BinaryHeap::new();
    // min heap, total, i, j, path remaning, direction as following
    // - 0 -
    // 1 * 3
    // - 2 -
    heap.push(Reverse((input[0][1], 0, 1, 1, 3)));
    heap.push(Reverse((input[1][0], 1, 0, 1, 2)));
    while !heap.is_empty() {
        let (total, i, j, len, dir) = heap.pop().unwrap().0;
        if i == input.len() - 1 && j == input[0].len() - 1 {
            if part2 && len < 4 {
                continue;
            }
            return total as i32;
        }
        for d in 0..4 {
            let (newi, newj) = (
                (i as i32 + (d - 1) % 2) as usize,
                (j as i32 + (d - 2) % 2) as usize,
            );
            if d == (dir + 2) % 4
                || newi >= input.len()
                || newj >= input[0].len()
                || d == dir && len == if part2 { 10 } else { 3 }
                || part2 && d != dir && len < 4
            {
                continue;
            }
            if vis.insert((d as usize, if d == dir { len + 1 } else { 1 }, newi, newj)) {
                heap.push(Reverse((
                    total + input[newi][newj],
                    newi,
                    newj,
                    if d == dir { len + 1 } else { 1 },
                    d,
                )));
            }
        }
    }
    0
}
fn part1(input: &str) -> i32 {
    solve(input, false)
}

fn part2(input: &str) -> i32 {
    solve(input, true)
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
