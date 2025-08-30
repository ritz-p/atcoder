use proconio::input;
fn main() {
    input! {
        mut rt: isize,
        mut ct: isize,
        mut ra: isize,
        mut ca: isize,
        n: usize,
        m: usize,
        l: usize,
        sa: [(char,isize);m],
        tb: [(char,isize);l],
    };
    let mut rx = rt - ra;
    let mut cy = ct - ca;
    let mut i = 0;
    let mut j = 0;
    let mut sc = sa[0].1;
    let mut tc = tb[0].1;
    let mut sv = dir(sa[0].0);
    let mut tv = dir(tb[0].0);
    let mut res = 0;
    while i < m && j < l {
        let len = sc.min(tc);
        let x = sv.0 - tv.0;
        let y = sv.1 - tv.1;

        res += count(rx, cy, x, y, len);
        rx += x * len;
        cy += y * len;
        sc -= len;
        tc -= len;
        if sc == 0 {
            i += 1;
            if i < m {
                let (s, a) = sa[i];
                sv = dir(s);
                sc = a;
            }
        }
        if tc == 0 {
            j += 1;
            if j < l {
                let (t, b) = tb[j];
                tv = dir(t);
                tc = b;
            }
        }
    }
    println!("{}", res);
}

fn dir(c: char) -> (isize, isize) {
    match c {
        'U' => (0, -1),
        'D' => (0, 1),
        'R' => (1, 0),
        'L' => (-1, 0),
        _ => unreachable!(),
    }
}

fn count(rx: isize, cy: isize, x: isize, y: isize, len: isize) -> isize {
    if x == 0 && y == 0 {
        if rx == 0 && cy == 0 {
            return len;
        } else {
            return 0;
        }
    }
    let mut to: Option<isize> = None;
    if x == 0 {
        if rx != 0 {
            return 0;
        }
    } else {
        let current = -rx;
        if current % x != 0 {
            return 0;
        }
        let t = current / x;
        to = Some(t);
    }
    if y == 0 {
        if cy != 0 {
            return 0;
        }
    } else {
        let current = -cy;
        if current % y != 0 {
            return 0;
        }
        let t2 = current / y;
        if let Some(t1) = to {
            if t1 != t2 {
                return 0;
            }
        }
        to = Some(t2);
    }
    if let Some(v) = to {
        if v >= 1 && v <= len {
            return 1;
        } else {
            return 0;
        }
    } else {
        return 0;
    }
}
