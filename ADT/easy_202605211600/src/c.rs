use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        mut m: usize
    };
    let mut n = vec![];

    loop {
        let mut current = 0;
        while 3_usize.pow(current) <= m {
            current += 1;
        }
        if 3_usize.pow(current) > m {
            current -= 1;
        }
        n.push(current);
        m -= 3_usize.pow(current);
        if m == 0 {
            break;
        }
    }

    println!("{}", n.len());
    println!("{}", n.iter().join(" "));
}
