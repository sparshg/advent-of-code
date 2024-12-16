use itertools::Itertools;
use std::collections::HashSet;

fn get_grid_moves(input: &str, part2: bool) -> (Vec<Vec<char>>, Vec<char>) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let grid = if part2 {
        grid.replace('.', "..")
            .replace('O', "[]")
            .replace('#', "##")
            .replace('@', "@.")
    } else {
        grid.to_string()
    };
    let grid = grid.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let moves = moves.chars().filter(|&c| c != '\n').collect_vec();
    (grid, moves)
}

fn find_start(grid: &Vec<Vec<char>>) -> (i32, i32) {
    grid.iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|&c| c == '@')
                .map(|j| (i as i32, j as i32))
        })
        .unwrap()
}

fn get_dir(mov: char) -> (i32, i32) {
    let (dx, dy) = match mov {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => unreachable!(),
    };
    (dx, dy)
}

fn get_empty_space(
    x: i32,
    dx: i32,
    y: i32,
    dy: i32,
    grid: &Vec<Vec<char>>,
) -> Option<(usize, usize)> {
    let mut i = 1;
    let (nx, ny) = loop {
        let (nx, ny) = ((x + i * dx) as usize, (y + i * dy) as usize);
        match grid.get(nx).and_then(|row| row.get(ny)) {
            Some(&'#') | None => return None,
            Some(&'.') => break (nx, ny),
            _ => {}
        }
        i += 1;
    };
    Some((nx, ny))
}

fn get_gps(grid: Vec<Vec<char>>, find: char) -> i32 {
    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter_map(|(i, j)| (grid[i][j] == find).then_some(100 * i + j))
        .sum::<usize>() as i32
}

fn part1(input: &str) -> i32 {
    let (mut grid, moves) = get_grid_moves(input, false);
    let (mut x, mut y) = find_start(&grid);
    'label: for mov in moves {
        let (dx, dy) = get_dir(mov);
        let Some((nx, ny)) = get_empty_space(x, dx, y, dy, &grid) else {
            continue 'label;
        };
        let (ax, ay) = ((x + dx) as usize, (y + dy) as usize);
        (grid[ax][ay], grid[nx][ny]) = (grid[nx][ny], grid[ax][ay]);
        (grid[x as usize][y as usize], grid[ax][ay]) = (grid[ax][ay], grid[x as usize][y as usize]);
        (x, y) = (ax as i32, ay as i32);
    }
    get_gps(grid, 'O')
}

fn part2(input: &str) -> i32 {
    let (mut grid, moves) = get_grid_moves(input, true);
    let (mut x, mut y) = find_start(&grid);
    'label: for mov in moves {
        let (dx, dy) = get_dir(mov);
        match mov {
            '^' | 'v' => {
                let mut boxes = HashSet::from([(x, y)]);
                let mut moved_boxes = Vec::new();
                while !boxes.is_empty() {
                    let mut new_boxes = HashSet::new();
                    for (bx, by) in boxes {
                        moved_boxes.push((bx, by));
                        let (tx, ty) = (bx + dx, by + dy);
                        new_boxes.insert((tx, ty));
                        match grid.get(tx as usize).and_then(|row| row.get(ty as usize)) {
                            Some(&'#') | None => continue 'label,
                            Some(&']') => new_boxes.insert((tx, ty - 1)),
                            Some(&'[') => new_boxes.insert((tx, ty + 1)),
                            Some(&'.') => new_boxes.remove(&(tx, ty)),
                            _ => false,
                        };
                    }
                    boxes = new_boxes;
                }
                for (bx, by) in moved_boxes.into_iter().rev() {
                    grid[(bx + dx) as usize][by as usize] = grid[bx as usize][by as usize];
                    grid[bx as usize][by as usize] = '.';
                }
            }
            _ => {
                let Some((nx, ny)) = get_empty_space(x, dx, y, dy, &grid) else {
                    continue 'label;
                };
                grid[x as usize][y as usize] = '.';
                grid[x as usize].remove(ny);
                grid[x as usize].insert((y + dy) as usize, '@');
            }
        }
        (x, y) = (x + dx, y + dy);
    }
    get_gps(grid, '[')
}

#[allow(unreachable_code)]
pub fn run(input: &str) -> Option<i32> {
    // return Some(part1(input));
    // return Some(part2(input));
    None
}
