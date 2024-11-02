use proconio::{input, marker::Chars};
use std::collections::{HashSet, VecDeque};
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    };

    let grid = s;
    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '.' {
                res += bfs(i, j, k, &grid, h, w);
            }
        }
    }

    println!("{}", res);
}

fn bfs(
    start_i: usize,
    start_j: usize,
    max_steps: usize,
    grid: &Vec<Vec<char>>,
    h: usize,
    w: usize,
) -> usize {
    let mut count = 0;
    let mut queue = VecDeque::new();

    let mut visited = HashSet::new();
    visited.insert((start_i, start_j));
    queue.push_back((start_i, start_j, visited.clone(), 0));

    while let Some((i, j, visited, steps)) = queue.pop_front() {
        if steps == max_steps {
            count += 1;
            continue;
        }

        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for &(dx, dy) in &directions {
            let ni = i as isize + dx;
            let nj = j as isize + dy;

            if ni >= 0 && ni < h as isize && nj >= 0 && nj < w as isize {
                let ni = ni as usize;
                let nj = nj as usize;
                if grid[ni][nj] == '.' && !visited.contains(&(ni, nj)) {
                    let mut new = visited.clone();
                    new.insert((ni, nj));
                    queue.push_back((ni, nj, new, steps + 1));
                }
            }
        }
    }

    count
}
