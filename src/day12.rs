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

fn mark_perimeter(groups: &[Vec<i32>]) -> Vec<i32> {
    let mut perimeter = vec![0; *groups.iter().flatten().max().unwrap() as usize + 1];
    for (i, j) in (0..groups.len()).cartesian_product(0..groups[0].len()) {
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

fn mark_perimeter_2(groups: &[Vec<i32>]) -> Vec<i32> {
    let mut perimeter = vec![0; *groups.iter().flatten().max().unwrap() as usize + 1];
    // add padding
    let mut groups = groups.to_owned();
    groups.insert(0, vec![-1; groups[0].len()]);
    groups.push(vec![-1; groups[0].len()]);
    groups.iter_mut().for_each(|row| {
        row.insert(0, -1);
        row.push(-1);
    });
    for (i, j) in (1..groups.len()).cartesian_product(0..groups[0].len() - 1) {
        if groups[i][j + 1] != groups[i][j] {
            if j < groups[0].len() - 2
                && (groups[i - 1][j + 1] != groups[i][j + 1]
                    || groups[i - 1][j + 1] == groups[i - 1][j])
            {
                // println!("Left: {} {}", i, j + 1);
                perimeter[groups[i][j + 1] as usize] += 1;
            }
            if j > 0
                && (groups[i - 1][j] != groups[i][j] || groups[i - 1][j + 1] == groups[i - 1][j])
            {
                // println!("Right: {} {}", i, j);
                perimeter[groups[i][j] as usize] += 1;
            }
        }
        if groups[i - 1][j + 1] != groups[i][j + 1] {
            if i < groups.len() - 1
                && (groups[i][j + 1] != groups[i][j] || groups[i][j] == groups[i - 1][j])
            {
                // println!("Top: {} {}", i, j + 1);
                perimeter[groups[i][j + 1] as usize] += 1;
            }
            if i > 1
                && (groups[i - 1][j + 1] != groups[i - 1][j] || groups[i - 1][j] == groups[i][j])
            {
                // println!("Bottom: {} {}", i - 1, j + 1);
                perimeter[groups[i - 1][j + 1] as usize] += 1;
            }
        }
    }
    perimeter
}

fn solve(input: &str, perimeter: &dyn Fn(&[Vec<i32>]) -> Vec<i32>) -> i32 {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut groups = vec![vec![-1; grid[0].len()]; grid.len()];
    let mut group = 0;
    for (i, j) in (0..grid.len()).cartesian_product(0..grid[0].len()) {
        if groups[i][j] == -1 {
            mark_groups(i as i32, j as i32, grid[i][j], &grid, &mut groups, group);
            group += 1;
        }
    }
    let perimeter = perimeter(&groups);
    groups
        .into_iter()
        .flatten()
        .sorted()
        .group_by(|&g| g)
        .into_iter()
        .map(|(g, group)| perimeter[g as usize] * group.count() as i32)
        .sum()
}

fn part1(input: &str) -> i32 {
    solve(input, &mark_perimeter)
}

fn part2(input: &str) -> i32 {
    solve(input, &mark_perimeter_2)
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
