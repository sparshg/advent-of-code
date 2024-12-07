use std::collections::HashSet;

use itertools::Itertools;

fn solve(input: &str, part2: bool) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let (target, nums) = line.split_once(": ").unwrap();
            let target = target.parse::<u64>().unwrap();
            let nums = nums
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect_vec();
            let mut seen = HashSet::from([nums[0]]);
            for &num in nums[1..].iter() {
                let mut new_seen = HashSet::new();
                for n in seen.into_iter() {
                    let mut ops = vec![n + num, n * num];
                    if part2 {
                        ops.push(n * 10u64.pow((num as f64).log10().floor() as u32 + 1) + num);
                    }
                    new_seen.extend(ops.into_iter().filter(|&i| i <= target));
                }
                seen = new_seen;
            }
            seen.contains(&target).then(|| target)
        })
        .sum::<u64>()
}

fn part1(input: &str) -> i32 {
    println!("{}", solve(input, false));
    0
}

fn part2(input: &str) -> i32 {
    println!("{}", solve(input, true));
    0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    return Some(part2(input));
    None
}
