use std::collections::{HashSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        xyp: [(isize,isize,isize);n]
    };

    let mut xs = vec![];
    let mut ys = vec![];
    let mut power = vec![];
    for (x, y, p) in xyp {
        xs.push(x);
        ys.push(y);
        power.push(p);
    }
    let mut ng = -1;
    let mut ok = 4000000001;
    while ng + 1 != ok {
        let mid = (ng + ok) / 2;
        if check(mid, n, &xs, &ys, &power) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn movable(
    s: isize,
    from: usize,
    to: usize,
    x: &Vec<isize>,
    y: &Vec<isize>,
    power: &Vec<isize>,
) -> bool {
    return power[from] * s >= (x[from] - x[to]).abs() + (y[from] - y[to]).abs();
}

fn check(s: isize, n: usize, x: &Vec<isize>, y: &Vec<isize>, power: &Vec<isize>) -> bool {
    for start in 0..n {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        visited.insert(start);
        queue.push_back(start);
        while let Some(current) = queue.pop_front() {
            for to in 0..n {
                if to != current {
                    let value = movable(s, current, to, &x, &y, &power);
                    if value && !visited.contains(&to) {
                        visited.insert(to);
                        queue.push_back(to);
                    }
                }
            }
        }
        if visited.len() == n {
            return true;
        }
    }
    return false;
}
