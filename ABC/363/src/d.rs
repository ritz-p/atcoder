use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize
    };
    let mut v = vec![1, 1];
    let mut current = 10;
    let mut is_odd = true;
    if n < 11 {
        println!("{}", n - 1);
        return;
    }
    loop {
        let plus = 9 * 10_usize.pow((v.len() - 2) as u32);
        let l = v.len();
        if plus + current >= n {
            while current != n {
                v[l - 1] += 1;
                for j in 0..l - 1 {
                    if v[j] == 10 {
                        v[j + 1] += 1;
                        v[j] = 0;
                    }
                }
                current += 1;
            }
            if is_odd {
                println!("{}{}", v.iter().join(""), v.iter().rev().join(""));
            } else {
                println!(
                    "{}{}",
                    v.iter().join(""),
                    v.iter().take(l - 1).rev().join("")
                );
            }
            break;
        } else {
            if is_odd {
                v.push(0);
                v[0] = 1;
                for i in 1..v.len() {
                    v[i] = 0;
                }
                is_odd = false;
            } else {
                is_odd = true;
            }
            current += plus;
        }
    }
}
