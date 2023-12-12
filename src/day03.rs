use std::collections::HashMap;

use itertools::Itertools;

fn part1(input: &str) -> i32 {
    let input = input.lines().map(|x| x.chars().collect_vec()).collect_vec();
    let mut sum = 0;
    for (i, line) in input.iter().enumerate() {
        let mut num = 0;
        let mut valid = false;
        for (j, c) in line.iter().enumerate() {
            if c.is_numeric() {
                num = num * 10 + c.to_digit(10).unwrap();
                valid |= (-1..=1)
                    .cartesian_product(-1..=1)
                    .filter_map(|(di, dj)| {
                        input
                            .get((i as i32 + di) as usize)
                            .and_then(|row| row.get((j as i32 + dj) as usize))
                    })
                    .any(|&x| !x.is_numeric() && x != '.');
            } else {
                if valid {
                    sum += num
                };
                valid = false;
                num = 0;
            }
        }
        if valid {
            sum += num
        };
    }
    sum as i32
}

fn part2(input: &str) -> i32 {
    let input = input.lines().map(|x| x.chars().collect_vec()).collect_vec();
    let mut stars = HashMap::new();
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            let mut adj = 0;
            for k in [-1, 1] {
                let it = (-1..=1).map(|dj| {
                    input
                        .get((i as i32 + k) as usize)
                        .and_then(|row| row.get((j as i32 + dj) as usize))
                });
                if it.clone().flatten().any(|x| x.is_numeric()) {
                    adj += 1;
                }
                if it
                    .zip([Some(()), None, Some(())])
                    .all(|(x, y)| x.is_some_and(|x| x.is_numeric()) == y.is_some())
                {
                    adj += 1;
                }
                if line
                    .get((j as i32 + k) as usize)
                    .is_some_and(|x| x.is_numeric())
                {
                    adj += 1
                }
            }
            if *c == '*' && adj == 2 {
                stars.insert((i, j), 1);
            }
        }
    }

    for (i, line) in input.iter().enumerate() {
        let mut num = 0;
        let mut valid = false;
        let mut st = (0, 0);
        for (j, c) in line.iter().enumerate() {
            if c.is_numeric() {
                num = num * 10 + c.to_digit(10).unwrap();
                valid |= (-1..=1)
                    .cartesian_product(-1..=1)
                    .map(|(di, dj)| {
                        input
                            .get((i as i32 + di) as usize)
                            .and_then(|row| row.get((j as i32 + dj) as usize))
                            .map(|_| ((i as i32 + di) as usize, (j as i32 + dj) as usize))
                    })
                    .any(|x| {
                        x.is_some_and(|x| {
                            if stars.contains_key(&x) {
                                st = x;
                                return true;
                            }
                            false
                        })
                    });
            } else {
                if valid {
                    *stars.get_mut(&st).unwrap() *= num;
                };
                valid = false;
                num = 0;
            }
        }
        if valid {
            *stars.get_mut(&st).unwrap() *= num;
        };
    }
    stars.values().sum::<u32>() as i32
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
