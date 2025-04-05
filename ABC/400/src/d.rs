use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    let (sr, sc) = (a - 1, b - 1);
    let (gr, gc) = (c - 1, d - 1);
    let mut walls = vec![vec![usize::MAX; w]; h];
    let mut broken = vec![vec![usize::MAX; w]; h];
    walls[sr][sc] = 0;

    let mut deque = VecDeque::new();
    deque.push_back((sr, sc));

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some((row, col)) = deque.pop_front() {
        let current = walls[row][col];

        if (row, col) == (gr, gc) {
            println!("{}", current);
            return;
        }
        for &(dr, dc) in &directions {
            let nr = row as isize + dr;
            let nc = col as isize + dc;
            if !bounds(nr, nc, h, w) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if s[nr][nc] == '.' || broken[nr][nc] <= current {
                if walls[nr][nc] > current {
                    walls[nr][nc] = current;
                    deque.push_front((nr, nc));
                }
            }
        }
        let next = current + 1;

        if walls[row][col] > next {
            walls[row][col] = next;
            deque.push_back((row, col));
        }

        for &(dr, dc) in &directions {
            let r1 = row as isize + dr;
            let c1 = col as isize + dc;
            if bounds(r1, c1, h, w) {
                let (r1, c1) = (r1 as usize, c1 as usize);
                if s[r1][c1] == '#' && broken[r1][c1] > next {
                    broken[r1][c1] = next;
                }
            }
            let r2 = row as isize + 2 * dr;
            let c2 = col as isize + 2 * dc;
            if bounds(r2, c2, h, w) {
                let (r2, c2) = (r2 as usize, c2 as usize);
                if s[r2][c2] == '#' && broken[r2][c2] > next {
                    broken[r2][c2] = next;
                }
            }
        }
    }
    for i in 0..h {
        println!("{:?}", walls[i]);
    }
}

fn bounds(r: isize, c: isize, h: usize, w: usize) -> bool {
    r >= 0 && r < h as isize && c >= 0 && c < w as isize
}
