use itertools::Itertools;

fn mark_groups(
    i: i32,
    j: i32,
    c: char,
    grid: &Vec<Vec<char>>,
    groups: &mut Vec<Vec<i32>>,
    group: i32,
) {
    if groups[i as usize][j as usize] != -1 {
        return;
    }
    groups[i as usize][j as usize] = group;
    [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
        .into_iter()
        .filter(|&(i, j)| {
            let (i, j) = (i as usize, j as usize);
            (0..grid.len()).contains(&i) && (0..grid[0].len()).contains(&j) && grid[i][j] == c
        })
        .for_each(|(i, j)| mark_groups(i, j, c, grid, groups, group));
}

fn mark_perimeter(groups: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut perimeter = vec![0; *groups.iter().flatten().max().unwrap() as usize + 1];
    for (i, j) in (0..groups.len())
        .into_iter()
        .cartesian_product(0..groups[0].len())
    {
        if j + 1 < groups[0].len() && groups[i][j + 1] != groups[i][j] {
            perimeter[groups[i][j] as usize] += 1;
            perimeter[groups[i][j + 1] as usize] += 1;
        }
        if i + 1 < groups.len() && groups[i + 1][j] != groups[i][j] {
            perimeter[groups[i][j] as usize] += 1;
            perimeter[groups[i + 1][j] as usize] += 1;
        }
        for _ in [
            i == 0,
            i == groups.len() - 1,
            j == 0,
            j == groups[0].len() - 1,
        ]
        .into_iter()
        .filter(|&b| b)
        {
            perimeter[groups[i][j] as usize] += 1;
        }
    }
    perimeter
}

fn part1(input: &str) -> i32 {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut groups = vec![vec![-1; grid[0].len()]; grid.len()];
    let mut group = 0;
    for (i, j) in (0..grid.len())
        .into_iter()
        .cartesian_product(0..grid[0].len())
    {
        if groups[i][j] == -1 {
            mark_groups(i as i32, j as i32, grid[i][j], &grid, &mut groups, group);
            group += 1;
        }
    }
    let perimeter = mark_perimeter(&groups);
    groups
        .into_iter()
        .flatten()
        .sorted()
        .group_by(|&g| g)
        .into_iter()
        .map(|(g, group)| perimeter[g as usize] * group.count() as i32)
        .sum()
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
