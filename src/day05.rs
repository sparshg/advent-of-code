use std::collections::HashMap;

use itertools::Itertools;

fn rules_and_updates(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|x| x.split_once('|').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .into_group_map();
    let updates = updates
        .lines()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect_vec())
        .collect_vec();
    (rules, updates)
}
fn part1(input: &str) -> i32 {
    let (rules, updates) = rules_and_updates(input);
    updates
        .iter()
        .filter_map(|nums| {
            nums[1..]
                .iter()
                .enumerate()
                .all(|(i, a)| rules.get(a).map_or(false, |v| !v.contains(&nums[i])))
                .then_some(nums[nums.len() / 2])
        })
        .sum()
}

fn repair(i: usize, nums: &mut Vec<i32>, rules: &HashMap<i32, Vec<i32>>) {
    let mut j = i as i32 - 1;
    while j >= 0
        && rules
            .get(&nums[i])
            .map_or(false, |v| v.contains(&nums[j as usize]))
    {
        j -= 1;
    }
    let num = nums.remove(i);
    nums.insert((j + 1) as usize, num);
}

fn part2(input: &str) -> i32 {
    let (rules, mut updates) = rules_and_updates(input);
    updates
        .iter_mut()
        .filter_map(|nums| {
            let mut repaired = false;
            for i in 1..nums.len() {
                if rules
                    .get(&nums[i])
                    .map_or(false, |v| v.contains(&nums[i - 1]))
                {
                    repaired = true;
                    repair(i, nums, &rules);
                }
            }
            repaired.then_some(nums[nums.len() / 2])
        })
        .sum()
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
