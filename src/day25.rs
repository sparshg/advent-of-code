use itertools::Itertools;

fn part1(input: &str) -> i32 {
    let lock_keys = input
        .split("\n\n")
        .map(|x| {
            let mut heights = [0; 5];
            for line in x.lines() {
                for (i, c) in line.chars().enumerate() {
                    heights[i] += (c == '#') as i32;
                }
            }
            (x.lines().next().unwrap() == "#####", heights)
        })
        .into_group_map();
    lock_keys[&false]
        .iter()
        .cartesian_product(&lock_keys[&true])
        .map(|(k, l)| k.iter().zip(l.iter()).map(|(k, l)| k + l).all(|x| x <= 7))
        .filter(|&x| x)
        .count() as i32
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
