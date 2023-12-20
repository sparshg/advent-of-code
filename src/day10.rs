
use std::{collections::HashSet};

use itertools::Itertools;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn inverse(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
    fn add(&self, pos: &mut (usize, usize)) {
        match self {
            Dir::Up => pos.0 -= 1,
            Dir::Down => pos.0 += 1,
            Dir::Left => pos.1 -= 1,
            Dir::Right => pos.1 += 1,
        }
    }
}

struct Pipe(Dir, Dir);

impl Pipe {
    fn next(&self, inc: &Dir) -> Result<Dir, ()> {
        if self.0 == inc.inverse() {
            return Ok(self.1);
        }
        if self.1 == inc.inverse() {
            return Ok(self.0);
        }
        // invalid move
        Err(())
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipe(Dir::Up, Dir::Down),
            '-' => Pipe(Dir::Left, Dir::Right),
            'L' => Pipe(Dir::Up, Dir::Right),
            'J' => Pipe(Dir::Up, Dir::Left),
            'F' => Pipe(Dir::Down, Dir::Right),
            '7' => Pipe(Dir::Down, Dir::Left),
            _ => panic!("Unknown Pipe"),
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut start = (0, 0);
    let pipes = input
        .lines()
        .enumerate()
        .map(|(i, x)| {
            x.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => None,
                    'S' => {
                        start = (i, j);
                        None
                    }
                    _ => Some(Pipe::from(c)),
                })
                .collect_vec()
        })
        .collect_vec();

    for dir in &mut [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
        let mut pos = start;
        dir.add(&mut pos);
        let mut set = HashSet::from([start]);
        let mut steps = 1;
        while let Some(pipe) = &pipes[pos.0][pos.1] {
            steps += 1;
            if set.contains(&pos) {
                break;
            }
            set.insert(pos);
            match pipe.next(dir) {
                Ok(d) => {
                    *dir = d;
                    dir.add(&mut pos);
                }
                _ => break,
            }
        }
        if pos == start {
            return steps / 2;
        }
    }
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
