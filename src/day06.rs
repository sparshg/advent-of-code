use std::collections::HashSet;

use itertools::Itertools;

fn check_loop(mut x: i32, mut y: i32, dir: (i32, i32, char), grid: &Vec<Vec<char>>) -> bool {
    let mut prev = grid[x as usize][y as usize];
    while let Some(c) = grid.get(x as usize).and_then(|row| row.get(y as usize)) {
        if c == &dir.2 || c == &'#' && transform_dir(dir).2 == prev {
            return true;
        }
        x += dir.1;
        y += dir.0;
        prev = *c;
    }
    false
}

fn part1(input: &str) -> i32 {
    let mut grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let (mut x, mut y) = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|&c| c == '^')
                .map(|j| (i as i32, j as i32))
        })
        .unwrap();
    let mut dir = (0, -1);
    while let Some(c) = grid
        .get_mut(x as usize)
        .and_then(|row| row.get_mut(y as usize))
    {
        if *c != '#' {
            *c = 'v';
        } else {
            x -= dir.1;
            y -= dir.0;
            dir = (-dir.1, dir.0);
        }
        x += dir.1;
        y += dir.0;
    }
    grid.iter().flatten().filter(|&&c| c == 'v').count() as i32
}

fn transform_dir(dir: (i32, i32, char)) -> (i32, i32, char) {
    match dir.2 {
        '<' => (-dir.1, dir.0, '^'),
        '^' => (-dir.1, dir.0, '>'),
        '>' => (-dir.1, dir.0, 'v'),
        'v' => (-dir.1, dir.0, '<'),
        x => (-dir.1, dir.0, x),
    }
}

fn part2(input: &str) -> i32 {
    let mut grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut blockers = HashSet::new();
    let (mut x, mut y) = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|&c| c == '^')
                .map(|j| (i as i32, j as i32))
        })
        .unwrap();
    let mut dir = (0, -1, '^');
    while let Some(c) = grid
        .get_mut(x as usize)
        .and_then(|row| row.get_mut(y as usize))
    {
        if *c != '#' {
            if check_loop(x, y, transform_dir(dir), &grid) {
                // print grid
                // for row in &grid {
                //     println!("{}", row.iter().collect::<String>());
                // }
                // println!("x: {}, y: {}\n", x + dir.1, y + dir.0);
                // check_loop(x, y, (-dir.1, dir.0), &grid);
                blockers.insert((x + dir.1, y + dir.0));
            }
            grid[x as usize][y as usize] = dir.2;
        } else {
            x -= dir.1;
            y -= dir.0;
            dir = transform_dir(dir);
        }
        x += dir.1;
        y += dir.0;
    }
    // dbg!(&blockers);
    blockers.len() as i32
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
