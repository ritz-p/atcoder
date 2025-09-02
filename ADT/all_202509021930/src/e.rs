use proconio::input;
fn main() {
    input! {
        n: usize,
        mut h: [usize;n]
    };
    let mut res = 0;

    for mut e in h {
        while e > 0 {
            match e {
                0 => {}
                1 => {
                    res += 1;
                    e = 0;
                }
                2 => {
                    if res % 3 == 2 {
                        res += 1;
                    } else {
                        res += 2;
                    }
                    e = 0;
                }
                3 => {
                    if res % 3 == 2 {
                        res += 1;
                    } else if res % 3 == 1 {
                        res += 2;
                    } else {
                        res += 3;
                    }
                    e = 0;
                }
                4 => {
                    if res % 3 == 0 {
                        res += 3;
                    } else if res % 3 == 1 {
                        res += 2;
                    } else if res % 3 == 2 {
                        res += 2;
                    }
                    e = 0;
                }
                _ => {
                    res += e / 5 * 3;
                    e %= 5;
                }
            }
        }
    }
    println!("{}", res);
}
