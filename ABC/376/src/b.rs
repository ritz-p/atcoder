use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        ht: [(char,usize);q]
    };
    let mut res = 0;
    let mut left = 1;
    let mut right = 2;
    for (h, t) in ht {
        if h == 'L' {
            let moves = min_moves(n, left, t, right);
            res += moves;
            left = t;
        } else {
            let moves = min_moves(n, right, t, left);
            res += moves;
            right = t;
        }
    }

    println!("{}", res);
}

fn min_moves(n: usize, start: usize, target: usize, avoid: usize) -> usize {
    let cw_distance = distance(n, start, target, avoid);
    let ccw_distance = distance_rev(n, start, target, avoid);
    cw_distance.min(ccw_distance)
}
fn distance(n: usize, start: usize, target: usize, avoid: usize) -> usize {
    if start == target {
        return 0;
    }
    let mut pos = start;
    let mut moves = 0;
    loop {
        pos = pos % n + 1;
        moves += 1;
        if pos == avoid {
            return usize::MAX;
        }
        if pos == target {
            return moves;
        }
    }
}

fn distance_rev(n: usize, start: usize, target: usize, avoid: usize) -> usize {
    if start == target {
        return 0;
    }
    let mut pos = start;
    let mut moves = 0;
    loop {
        pos = if pos == 1 { n } else { pos - 1 };
        moves += 1;
        if pos == avoid {
            return usize::MAX;
        }
        if pos == target {
            return moves;
        }
    }
}
