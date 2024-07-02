use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(isize,isize);n]
    };

    let mut res = solve(true, true, &ab);
    res = res.max(solve(true, false, &ab));
    res = res.max(solve(false, true, &ab));
    res = res.max(solve(false, false, &ab));

    println!("{}", res);
}

fn solve(front: bool, back: bool, ab: &Vec<(isize, isize)>) -> usize {
    let mut sum = 0;
    for (a, b) in ab {
        let ac = if !front { *a } else { -a };

        let bc = if !back { *b } else { -b };
        if ac + bc >= 0 {
            sum += ac + bc
        }
    }
    sum as usize
}
