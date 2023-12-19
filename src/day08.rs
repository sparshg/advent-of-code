use std::collections::{hash_map::RandomState, HashMap};

use itertools::Itertools;

fn part1(input: &str) -> i32 {
    let mut inst = input.lines().next().unwrap().chars().cycle();
    let network: HashMap<&str, (&str, &str), RandomState> = HashMap::from_iter(
        input
            .lines()
            .skip(2)
            .map(|x| x.split_once(" = ").unwrap())
            .map(|(x, y)| (x, y[1..y.len() - 1].split_once(", ").unwrap())),
    );
    let mut at = "AAA";
    let mut count = 0;
    while at != "ZZZ" {
        at = if inst.next().unwrap() == 'L' {
            network[&at].0
        } else {
            network[&at].1
        };
        count += 1;
    }
    count
}

fn part2(input: &str) -> i32 {
    let mut inst = input.lines().next().unwrap().chars().cycle();
    let network: HashMap<&str, (&str, &str), RandomState> = HashMap::from_iter(
        input
            .lines()
            .skip(2)
            .map(|x| x.split_once(" = ").unwrap())
            .map(|(x, y)| (x, y[1..y.len() - 1].split_once(", ").unwrap())),
    );
    let mut nodes = network
        .keys()
        .filter(|&x| x.ends_with('A'))
        .map(|x| (x, 0))
        .collect_vec();
    while nodes.iter().filter(|x| !x.0.ends_with('Z')).count() > 0 {
        let inst = inst.next().unwrap();
        for x in nodes.iter_mut().filter(|y| !y.0.ends_with('Z')) {
            *x = (
                if inst == 'L' {
                    &network[*x.0].0
                } else {
                    &network[*x.0].1
                },
                x.1 + 1,
            )
        }
    }
    println!("{}", nodes.iter().map(|(_, x)| *x as i64).fold(1, lcm));
    0
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a / gcd(a, b)) * b
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
