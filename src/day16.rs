use std::{collections::BinaryHeap, i32::MAX};

use itertools::Itertools;

fn find(grid: &[Vec<char>], c: char) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&x| x == c).map(|j| (i, j)))
        .unwrap()
}

fn dijkstra(grid: &[Vec<char>], sx: usize, sy: usize, ex: usize, ey: usize) -> (i32, i32) {
    // part 1
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut dist = vec![vec![[std::i32::MAX; 4]; grid[0].len()]; grid.len()];
    let mut queue = BinaryHeap::from([(0, sx, sy, 0)]);
    dist[sx][sy] = [0; 4];
    let mut reached = 0;
    while let Some((d, x, y, dir)) = queue.pop() {
        if -d > dist[x][y][dir] {
            continue;
        }
        if (x, y) == (ex, ey) {
            reached = dir;
            break;
        }

        for (i, (dx, dy)) in directions.into_iter().enumerate() {
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            if !((0..grid.len()).contains(&nx) && (0..grid[0].len()).contains(&ny))
                || grid[nx][ny] == '#'
            {
                continue;
            }
            let cost = dist[x][y][dir] + if i == dir { 1 } else { 1001 };
            if cost <= dist[nx][ny][i] {
                dist[nx][ny][i] = cost;
                queue.push((-cost, nx, ny, i));
            }
        }
    }
    // part 2
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut stack = vec![(
        ex as i32 - directions[reached].0,
        ey as i32 - directions[reached].1,
        reached,
    )];
    visited[ex][ey] = true;
    while let Some((x, y, dir)) = stack.pop() {
        visited[x as usize][y as usize] = true;
        if (x, y) == (sx as i32, sy as i32) {
            continue;
        }
        let cost = dist[(x + directions[dir].0) as usize][(y + directions[dir].1) as usize][dir];
        for dir in dist[x as usize][y as usize]
            .iter()
            .enumerate()
            .filter_map(|(i, &d)| {
                (i == dir && d == cost - 1 || i != dir && d == cost - 1001).then_some(i)
            })
        {
            let (dx, dy) = directions[dir];
            let (nx, ny) = (x - dx, y - dy);
            stack.push((nx, ny, dir));
        }
    }

    (
        *dist[ex][ey].iter().min().unwrap(),
        visited.iter().flatten().filter(|&&x| x).count() as i32,
    )
}

fn solve(input: &str) -> (i32, i32) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let (sx, sy) = find(&grid, 'S');
    let (ex, ey) = find(&grid, 'E');
    dijkstra(&grid, sx, sy, ex, ey)
}

fn part1(input: &str) -> i32 {
    solve(input).0
}

fn part2(input: &str) -> i32 {
    solve(input).1
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    return Some(part2(input));
    None
}
