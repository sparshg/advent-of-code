use itertools::Itertools;

fn search(x: i32, y: i32, grid: &Vec<Vec<u32>>, seen: &mut Option<Vec<(i32, i32)>>) -> u32 {
    if grid[x as usize][y as usize] == 9 {
        if let Some(seen) = seen {
            if seen.contains(&(x, y)) {
                return 0;
            }
            seen.push((x, y));
        }
        return 1;
    }
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter(|(i, j)| {
            let (i, j) = (*i as usize, *j as usize);
            (0..grid.len()).contains(&i)
                && (0..grid[0].len()).contains(&j)
                && grid[i][j] == grid[x as usize][y as usize] + 1
        })
        .map(|(i, j)| search(i, j, grid, seen))
        .sum()
}

fn solve(input: &str, part2: bool) -> i32 {
    let grid = &input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == 0)
                .map(move |(j, _)| {
                    search(i as i32, j as i32, grid, &mut (!part2).then_some(vec![]))
                })
        })
        .sum::<u32>() as i32
}

fn part1(input: &str) -> i32 {
    solve(input, false)
}

fn part2(input: &str) -> i32 {
    solve(input, true)
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    return Some(part2(input));
    None
}
