use itertools::Itertools;

fn solve(input: &str, range: impl Iterator<Item = i32> + Clone) -> i32 {
    let mut grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let groups = grid
        .iter()
        .enumerate()
        .flat_map(|(i, r)| {
            r.iter()
                .enumerate()
                .filter(|(_, x)| **x != '.')
                .map(move |(j, &c)| (c, (i as i32, j as i32)))
        })
        .into_grouping_map()
        .collect::<Vec<_>>();
    for (antenna, positions) in groups {
        for ((x1, y1), (x2, y2)) in positions.iter().tuple_combinations() {
            let (dx, dy) = (x2 - x1, y2 - y1);
            for (i, j) in range.clone().map(|i| (x1 + i * dx, y1 + i * dy)) {
                let (i, j) = (i as usize, j as usize);
                if i < grid.len() && j < grid[0].len() {
                    grid[i][j] = '#';
                }
            }
        }
    }
    grid.iter().flatten().filter(|&&c| c == '#').count() as i32
}
fn part1(input: &str) -> i32 {
    solve(input, [2, -1].into_iter())
}

fn part2(input: &str) -> i32 {
    let size = input
        .lines()
        .next()
        .unwrap()
        .len()
        .max(input.lines().count()) as i32;
    solve(input, -size..size)
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    return Some(part2(input));
    None
}
