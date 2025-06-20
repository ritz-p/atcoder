use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let mut aset = HashSet::new();

    let mut av = vec![a % 10];
    aset.insert(a % 10);
    let mut current = 1;
    loop {
        let next = (a * av[current - 1]) % 10;
        if !aset.contains(&next) {
            aset.insert(next);
            av.push(next);
            current += 1;
        } else {
            break;
        }
    }
    let l = av.len();
    let mut m = 1;
    let mut base = b % l;
    let mut exp = c;
    while exp > 0 {
        if exp % 2 == 1 {
            m = m * base % l;
        }
        base = base * base % l;
        exp /= 2;
    }
    if m == 0 {
        m = l;
    }
    println!("{}", av[m - 1]);
}
