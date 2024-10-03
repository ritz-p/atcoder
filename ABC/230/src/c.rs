use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        _n: usize,
        a: isize,
        b: isize,
        p: isize,
        q: isize,
        r: isize,
        s: isize,
    };
    let sum = a + b;
    let diff = a - b;

    let mut v = vec![vec!['.'; s as usize - r as usize + 1]; q as usize - p as usize+ 1];
    for i in p..=q {
        for j in r..=s {
            if (i + j) as isize == sum{
                v[i as usize - p as usize][j as usize - r as usize] = '#';
            }
            if i as isize - j as isize == diff{
                v[i as usize - p as usize][j as usize - r as usize] = '#';
            }
        }
    }

    for i in v {
        println!("{}", i.iter().join(""));
    }
}
