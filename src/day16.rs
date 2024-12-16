use std::{collections::BinaryHeap, i32::MAX};

use itertools::Itertools;

fn find(grid: &[Vec<char>], c: char) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&x| x == c).map(|j| (i, j)))
        .unwrap()
}

fn dijkstra(grid: &[Vec<char>], sx: usize, sy: usize, ex: usize, ey: usize) -> i32 {
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut dist = vec![vec![[std::i32::MAX; 4]; grid[0].len()]; grid.len()];
    let mut queue = BinaryHeap::from([(0, sx, sy, 0)]);
    dist[sx][sy] = [0; 4];
    let mut reached = vec![];
    while let Some((d, x, y, dir)) = queue.pop() {
        if -d > dist[x][y][dir] {
            continue;
        }
        if (x, y) == (ex, ey) {
            reached.push(dir);
            break;
        }

        for (i, (dx, dy)) in directions.into_iter().enumerate() {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if !((0..grid.len() as i32).contains(&nx) && (0..grid[0].len() as i32).contains(&ny))
                || grid[nx as usize][ny as usize] == '#'
            {
                continue;
            }
            let cost = dist[x][y][dir] + if i == dir { 1 } else { 1001 };
            if cost <= dist[nx as usize][ny as usize][i] {
                dist[nx as usize][ny as usize][i] = cost;
                queue.push((-cost, nx as usize, ny as usize, i));
            }
        }
    }
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut stack = reached.iter().map(|&x| (ex, ey, x)).collect_vec();
    visited[ex][ey] = true;
    while let Some((x, y, dir)) = stack.pop() {
        dbg!((x, y, dist[x][y], dir));

        if (x, y) == (sx, sy) {
            continue;
        }
        let mut cost = *dist[x][y].iter().min().unwrap();
        if dist[x][y][dir] != cost {
            cost += 1000;
        }
        for dir in dist[x][y].iter().enumerate().filter_map(|(i, &d)| {
            (i == dir && d == cost || i != dir && d == cost - 1000).then_some(i)
        }) {
            let (dx, dy) = directions[dir];
            let (nx, ny) = ((x as i32 - dx) as usize, (y as i32 - dy) as usize);
            stack.push((nx, ny, dir));

            visited[nx][ny] = true;
        }
    }

    dbg!(&stack);

    for row in visited.iter() {
        println!(
            "{}",
            row.iter()
                .map(|&x| if x { '#' } else { '.' })
                .collect::<String>()
        );
    }
    dbg!(visited.iter().flatten().filter(|&&x| x).count());

    *dist[ex][ey].iter().min().unwrap()
}

fn part1(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let (sx, sy) = find(&grid, 'S');
    let (ex, ey) = find(&grid, 'E');
    dijkstra(&grid, sx, sy, ex, ey)
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
