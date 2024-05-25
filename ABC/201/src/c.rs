use proconio::{marker::Chars, *};

fn main() {
    input! {
        s: Chars,
    }

    let mut res = 0;
    for i in 0..10000 {
        let mut p = i;
        let mut flag = vec![false; 10];
        for _ in 0..4 {
            flag[p%10] = true;
            p /= 10;
        }

        let mut ok = true;
        for j in 0..10 {
            if s[j] == 'o' && !flag[j] {
                ok = false;
            }
            if s[j] == 'x' && flag[j] {
                ok = false;
            }
        }

        if ok {
            res += 1;
        }
    }

    println!("{res}")
}
