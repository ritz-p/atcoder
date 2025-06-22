use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize;q]
    };

    let mut v = vec![false; n];

    let mut current = 0;
    for query in a {
        let l = query - 1 > 0 && v[query - 2];
        let r = query < n && v[query];

        if !v[query - 1] {
            if !l && !r {
                current += 1;
            } else if l && r {
                current -= 1;
            }
            v[query - 1] = true;
        } else {
            if !l && !r {
                current -= 1;
            } else if l && r {
                current += 1;
            }
            v[query - 1] = false;
        }

        println!("{}", current);
    }
}
