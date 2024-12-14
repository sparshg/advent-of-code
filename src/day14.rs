use core::f32;

use itertools::Itertools;

fn get_robots(input: &str) -> impl Iterator<Item = (i32, i32, i32, i32)> + use<'_> {
    input.lines().map(|line| {
        line.split(&['=', ' ', ','])
            .filter_map(|x| x.parse::<i32>().ok())
            .collect_tuple()
            .unwrap()
    })
}

fn part1(input: &str) -> i32 {
    get_robots(input)
        .map(|(px, py, vx, vy)| {
            (
                (px + vx * 100).rem_euclid(101) - 101 / 2,
                (py + 100 * vy).rem_euclid(103) - 103 / 2,
            )
        })
        .filter(|&(x, y)| x != 0 && y != 0)
        .into_group_map_by(|&(x, y)| (x.signum(), y.signum()))
        .values()
        .map(Vec::len)
        .product::<usize>() as i32
}

fn entropy(grid: &[Vec<bool>]) -> f32 {
    (0..10)
        .cartesian_product(0..10)
        .map(|(i, j)| {
            let p = (0..10)
                .cartesian_product(0..10)
                .filter(|&(di, dj)| grid[10 * i + di][10 * j + dj])
                .count() as f32
                / 100.0;
            if p == 0.0 {
                0.0
            } else {
                -p * p.log2()
            }
        })
        .sum()
}

fn print_grid(grid: &[Vec<bool>]) {
    for row in grid.iter() {
        for &cell in row.iter() {
            print!("{}", if cell { '#' } else { '.' });
        }
        println!();
    }
}

fn part2(input: &str) -> i32 {
    let mut robots = get_robots(input).collect_vec();
    let mut beste = f32::INFINITY;
    for i in 1.. {
        let mut grid = vec![vec![false; 101]; 103];
        for (px, py, vx, vy) in robots.iter_mut() {
            *px = (*px + *vx).rem_euclid(101);
            *py = (*py + *vy).rem_euclid(103);
            grid[*py as usize][*px as usize] = true;
        }
        let e = entropy(&grid);
        if e < beste {
            beste = e;
            print_grid(&grid);
            println!("{}", i);
        }
    }
    0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
