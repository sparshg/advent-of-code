use std::{
    cmp::{max, min},
    collections::HashSet,
};

use itertools::Itertools;

#[derive(Hash, PartialEq, Eq)]
struct P(i32, i32);

fn part1(input: &str) -> i32 {
    let mut overlaps = HashSet::new();

    let input = input
        .lines()
        .map(|x| {
            x.split(|x| !char::is_numeric(x))
                .filter(|x| !x.is_empty())
                .skip(1)
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(a, b, c, d)| (P(a, b), P(a + c - 1, b + d - 1)))
        .collect_vec();

    for (i, rect) in input.iter().enumerate() {
        for other in input.iter().take(i) {
            let over = get_overlaps(&rect.0, &rect.1, &other.0, &other.1);
            if let Some(x) = over {
                for a in x.0 .0..=x.1 .0 {
                    for b in x.0 .1..=x.1 .1 {
                        overlaps.insert(P(a, b));
                    }
                }
            }
        }
    }

    // let a = get_overlaps(P(0, 0), P(4, 4), P(2, 4), P(6, 6));
    // if a.is_some() {
    //     a.unwrap().
    // }
    overlaps.len() as i32
}

fn part2(input: &str) -> i32 {
    0
}

fn get_overlaps(p1: &P, p2: &P, q1: &P, q2: &P) -> Option<(P, P)> {
    if (q1.1 < p2.1 && q2.1 > p1.1) && (q1.0 < p2.0 && q2.0 > p1.0) {
        return Some((
            P(max(q1.0, p1.0), max(q1.1, p1.1)),
            P(min(q2.0, p2.0), min(q2.1, p2.1)),
        ));
    }
    None
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    return Some(part1(input));
    // return Some(part2(input));
    None
}
