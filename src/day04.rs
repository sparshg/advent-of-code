use std::collections::HashMap;

use itertools::Itertools;
fn answer(input: &str) -> i32 {
    let a = input
        .lines()
        .map(|x| x.split(&['-', ' ', ':'][..]).collect_vec())
        .map(|mut x| {
            x[0] = &x[0][1..];
            x[4] = &x[4][..2];
            x
        })
        .sorted_unstable_by_key(|x| (x[0], x[1], x[2], x[3], x[4]))
        .collect_vec();
    let mut sleeps = HashMap::new();
    let mut maxminute: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    let mut guard = 0;
    let mut sleptat: Option<i32> = None;
    let default_map = HashMap::from_iter((0..60).zip(vec![0; 60].into_iter()));

    for info in &a {
        if info[6].chars().next().unwrap() == '#' {
            guard = info[6][1..].parse().unwrap();
            sleptat = None;
        } else if info[5] == "falls" {
            sleptat = info[4].parse().ok();
        } else if info[5] == "wakes" {
            let dur = info[4].parse::<i32>().unwrap() - sleptat.unwrap();

            sleeps.entry(guard).and_modify(|x| *x += dur).or_insert(dur);
            maxminute
                .entry(guard)
                .and_modify(|x| {
                    for i in sleptat.unwrap()..(dur + sleptat.unwrap()) {
                        *x.get_mut(&i).unwrap() += 1;
                    }
                })
                .or_insert_with(|| {
                    let mut default = default_map.clone();
                    for i in sleptat.unwrap()..(dur + sleptat.unwrap()) {
                        *default.get_mut(&i).unwrap() += 1;
                    }
                    default
                });
            sleptat = None;
        }
    }
    // part 1

    // guard = *sleeps.iter().max_by_key(|x| x.1).unwrap().0;
    // guard
    //     * maxminute
    //         .get(&guard)
    //         .unwrap()
    //         .iter()
    //         .max_by_key(|x| x.1)
    //         .unwrap()
    //         .0

    // part 2
    let part2 = maxminute
        .iter()
        .map(|(x, y)| (x, y.iter().max_by_key(|x| x.1).unwrap()))
        .max_by_key(|x| x.1 .1)
        .unwrap();
    part2.0 * part2.1 .0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(answer(input));
    None
}
