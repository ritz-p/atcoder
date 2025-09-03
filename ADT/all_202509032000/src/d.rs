use proconio::input;
fn main() {
    input! {
        q: usize,
        queries: [(usize,usize);q]
    };
    let mut v = vec![];
    for (qt, xk) in queries {
        match qt {
            1 => {
                v.push(xk);
            }
            2 => {
                let l = v.len();
                println!("{}", v[l - xk]);
            }
            _ => {}
        }
    }
}
