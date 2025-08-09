use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }
    let mut map = HashMap::new();
    let mut para: isize = 0;

    for i in 0..n {
        let (xi, yi) = xy[i];
        let mut lmap = HashMap::new();
        for j in 0..n {
            if i == j {
                continue;
            }
            let (xj, yj) = xy[j];
            let (dx, dy) = (xj - xi, yj - yi);
            let gcd = gcd(dx, dy);
            let mut rx = (dx / gcd) as i32;
            let mut ry = (dy / gcd) as i32;
            if rx < 0 || (rx == 0 && ry < 0) {
                rx = -rx;
                ry = -ry;
            }
            *lmap.entry((rx, ry)).or_insert(0) += 1;
        }
        for (d, f) in lmap {
            *map.entry(d).or_insert(0) += f;
            para += f * (f - 1) / 2;
        }
    }
    let mut slope: isize = 0;
    for (_, s) in &map {
        let m = *s / 2;
        slope += m * (m - 1) / 2;
    }
    let d = slope - para;
    let mut count = HashMap::new();
    for i in 0..n {
        for j in i + 1..n {
            let key = (xy[i].0 + xy[j].0, xy[i].1 + xy[j].1);
            *count.entry(key).or_insert(0) += 1;
        }
    }
    let mut p: isize = 0;
    for (_, c) in count {
        p += c * (c - 1) / 2;
    }
    let res = d - p;
    println!("{}", res);
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}
