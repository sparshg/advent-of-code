use std::{collections::BinaryHeap, i32};

use itertools::Itertools;

fn dijkstra(grid: &[Vec<bool>]) -> Option<i32> {
    let mut dist = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = BinaryHeap::from(vec![(0, (0, 0))]);
    visited[0][0] = true;
    while let Some((d, (x, y))) = queue.pop() {
        if (x, y) == (grid.len() - 1, grid[0].len() - 1) {
            return Some(-d);
        }
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            if (0..grid.len()).contains(&nx)
                && (0..grid[0].len()).contains(&ny)
                && grid[nx][ny]
                && !visited[nx][ny]
                && dist[nx][ny] > -d + 1
            {
                visited[nx][ny] = true;
                dist[x][y] = -d + 1;
                queue.push((d - 1, (nx, ny)));
            }
        }
    }
    None
}

fn part1(input: &str) -> i32 {
    let mut grid = vec![vec![true; 71]; 71];
    for (x, y) in input
        .split(&['\n', ','])
        .map(|x| x.parse::<usize>().unwrap())
        .take(2048)
        .tuples()
    {
        grid[x][y] = false;
    }
    dijkstra(&grid).unwrap()
}

fn part2(input: &str) -> i32 {
    let pos = input
        .split(&['\n', ','])
        .map(|x| x.parse::<usize>().unwrap())
        .tuples()
        .enumerate()
        .collect_vec();
    let index = pos.partition_point(|&(i, _)| {
        let mut grid = vec![vec![true; 71]; 71];
        for &(_, (x, y)) in pos.iter().take(i) {
            grid[x][y] = false;
        }
        dijkstra(&grid).is_some()
    });
    println!("{},{}", pos[index - 1].1 .0, pos[index - 1].1 .1);
    0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
