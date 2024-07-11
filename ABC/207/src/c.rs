use proconio::input;

fn main() {
    input! {
        n: usize,
        tlr: [(usize, usize, usize); n],
    }

    let mut res = 0;
    let mut v = vec![];
    for &(t, mut l, mut r) in &tlr {
        (l, r) = match t {
            1 => (2 * l, 2 * r),
            2 => (2 * l, 2 * r - 1),
            3 => (2 * l + 1, 2 * r),
            4 => (2 * l + 1, 2 * r - 1),
            _ => unreachable!(),
        };

        res += v
            .iter()
            .filter(|&&(other_l, other_r)| l <= other_r && r >= other_l)
            .count();

        v.push((l, r));
    }

    println!("{}", res);
}
