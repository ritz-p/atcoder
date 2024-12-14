use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: isize,
        a: [isize; n]
    };
    let mut sums = vec![0; n + 1];
    for i in 1..=n {
        sums[i] = sums[i - 1] + a[i - 1];
    }
    let sum = sums[n];
    if sum == 0 {
        let mut set = HashSet::new();
        for p in &sums {
            if set.contains(&(p - s)) {
                println!("Yes");
                return;
            }
            set.insert(*p);
        }
        println!("No");
        return;
    }
    let mut modulo = HashSet::new();
    for &p in &sums {
        modulo.insert(((p % sum) + sum) % sum);
    }
    for &p in &sums {
        let current = ((p - s) % sum + sum) % sum;
        if modulo.contains(&current) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
