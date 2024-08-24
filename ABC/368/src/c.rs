use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize;n]
    };
    let mut t = 0;
    for mut i in a {
        if (t + 1) % 3 == 0 {
            i -= 3;
            t += 1;
        } else if (t + 2) % 3 == 0 {
            i -= 1;
            t += 1;
            if i <= 0 {
                continue;
            }
            i -= 3;
            t += 1;
        }
        if i <= 0 {
            continue;
        }
        let current = i / 5;
        t += current * 3;
        if i % 5 == 0 {
            continue;
        } else if i % 5 == 1 {
            t += 1;
            continue;
        } else if i % 5 == 2 {
            if (t + 1) % 3 == 0 {
                t += 1;
            } else {
                t += 2;
            }
        } else if i % 5 == 3 {
            if (t + 1) % 3 == 0 {
                t += 1;
            } else {
                t += 3;
            }
        } else if i % 5 == 4 {
            if (t + 1) % 3 == 0 {
                t += 2;
            } else if (t + 2) % 3 == 0 {
                t += 2;
            } else if (t + 3) % 3 == 0 {
                t += 3;
            }
        }
    }

    println!("{}", t);
}
