use proconio::input;
fn main() {
    input! {
        n: usize,
    };

    let mut v = vec![vec!['#'; n]; n];
    for i in 0..n {
        let j = n - i - 1;
        if j >= i {
            if i % 2 == 0 {
                for k in i..=j {
                    for l in i..=j {
                        v[k][l] = '#';
                    }
                }
            } else {
                for k in i..=j {
                    for l in i..=j {
                        v[k][l] = '.';
                    }
                }
            }
        }
    }
    for i in 0..n {
        println!("{}", v[i].iter().collect::<String>());
    }
}
