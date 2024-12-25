use std::i32;

use itertools::Itertools;

fn dijkstra(grid: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> Option<i32> {
    // let mut dist = vec![vec![[i32::MAX, i32::MAX]; grid[0].len()]; grid.len()];
    // dist[start.0][start.1][1] = 0;
    let mut queue = vec![(start, 1)];
    let mut visited = vec![vec![[false, false]; grid[0].len()]; grid.len()];
    visited[start.0][start.1][1] = true;
    while let Some(((x, y), ability)) = queue.pop() {
        // println!("{} ln:{} col:{} {}", -d, x + 1, y + 1, ability);
        // if (x, y) == (1, 4) {
        //     println!("{} {} {} {}", x, y, ability, grid[x][y]);
        // }
        if (x, y) == end {
            // return Some(-d);
            // dbg!(-d);
        }
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            if (0..grid.len()).contains(&nx) && (0..grid[0].len()).contains(&ny) {
                if grid[nx][ny] != '#' {
                    if !visited[nx][ny][ability] {
                        visited[nx][ny][ability] = true;
                        queue.push(((nx, ny), ability));
                        // println!("{} {} {} {}", -d + 1, nx, ny, ability);
                    }
                } else if ability == 1 {
                    let (nx, ny) = ((x as i32 + 2 * dx) as usize, (y as i32 + 2 * dy) as usize);
                    if (0..grid.len()).contains(&nx)
                        && (0..grid[0].len()).contains(&ny)
                        && grid[nx][ny] != '#'
                        && !visited[nx][ny][0]
                    {
                        visited[nx][ny][0] = true;
                        queue.push(((nx, ny), 0));
                        // println!("{} {} {} {}", -d + 2, nx, ny, 0);
                    }
                }
            }
        }
    }
    None
    // Some(*dist[end.0][end.1].iter().min().unwrap())
}

fn find(grid: &[Vec<char>], c: char) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&x| x == c).map(|j| (i, j)))
        .unwrap()
}

fn part1(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let start = find(&grid, 'S');
    let end = find(&grid, 'E');
    dbg!(dijkstra(&grid, start, end));
    0
}

fn part2(input: &str) -> i32 {
    0
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
