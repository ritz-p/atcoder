use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars;h]
    };
    let mut queue = VecDeque::new();
    let mut start = (isize::MAX, isize::MAX);
    let mut set = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                start = (i as isize, j as isize);
                break;
            }
        }
        if start != (isize::MAX, isize::MAX) {
            break;
        }
    }
    queue.push_back((start.0, start.1, true, 0));
    let direction = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
    while let Some((s1, s2, flag, count)) = queue.pop_front() {
        for (x, y) in &direction {
            let (x, y) = (s1 + x, s2 + y);
            if x >= 0 && x < h as isize && y >= 0 && y < w as isize {
                match s[x as usize][y as usize] {
                    '.' | 'S' => {
                        if !set.contains(&(x, y, flag)) {
                            queue.push_back((x, y, flag, count + 1));
                            set.insert((x, y, flag));
                        }
                    }
                    'x' => {
                        if !flag && !set.contains(&(x, y, flag)) {
                            queue.push_back((x, y, flag, count + 1));
                            set.insert((x, y, flag));
                        }
                    }
                    'o' => {
                        if flag && !set.contains(&(x, y, flag)) {
                            queue.push_back((x, y, flag, count + 1));
                            set.insert((x, y, flag));
                        }
                    }
                    'G' => {
                        println!("{}", count + 1);
                        return;
                    }
                    '?' => {
                        if !set.contains(&(x, y, !flag)) {
                            queue.push_back((x, y, !flag, count + 1));
                            set.insert((x, y, !flag));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    println!("-1");
}
