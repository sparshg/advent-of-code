use std::collections::{hash_map::RandomState, HashMap};

use itertools::Itertools;

fn part1(input: &str) -> i32 {
    let workflows: HashMap<&str, Vec<(&str, Option<(char, char, i32)>)>, RandomState> =
        HashMap::from_iter(
            input
                .split_once("\n\n")
                .unwrap()
                .0
                .lines()
                .map(|l| l.split_once('{').unwrap())
                .map(|(x, v)| {
                    (
                        x,
                        v[..v.len() - 1]
                            .split(',')
                            .map(|e| {
                                e.split_once(':')
                                    .map(|(x, y)| {
                                        (
                                            y,
                                            Some((
                                                x.chars().nth(0).unwrap(),
                                                x.chars().nth(1).unwrap(),
                                                x[2..].parse::<i32>().unwrap(),
                                            )),
                                        )
                                    })
                                    .unwrap_or((e, None))
                            })
                            .collect_vec(),
                    )
                })
                .collect_vec(),
        );

    let mut res = 0;
    for (x, m, a, s) in input.split_once("\n\n").unwrap().1.lines().map(|l| {
        l[1..l.len() - 1]
            .split(',')
            .map(|e| e.split_once('=').unwrap().1.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap()
    }) {
        let mut node = "in";
        loop {
            for (to, condition) in &workflows[node] {
                if let Some((var, op, val)) = condition {
                    if match var {
                        'x' => x,
                        'm' => m,
                        'a' => a,
                        's' => s,
                        _ => unreachable!(),
                    }
                    .cmp(&val)
                        == match op {
                            '>' => std::cmp::Ordering::Greater,
                            '<' => std::cmp::Ordering::Less,
                            _ => unreachable!(),
                        }
                    {
                        node = to;
                        break;
                    }
                } else {
                    node = to;
                }
            }
            match node {
                "A" => {
                    res += x + m + a + s;
                    break;
                }
                "R" => break,
                _ => {}
            }
        }
    }
    res
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
