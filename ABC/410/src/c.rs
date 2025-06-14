use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut v = (1..=n).collect::<Vec<usize>>();
    let mut current = 0;
    for _i in 0..q {
        input! {
            query: usize,
        };
        match query {
            1 => {
                input! {
                    p: usize,
                    x: usize,
                };
                v[(p - 1 + current) % n] = x;
            }
            2 => {
                input! {
                    p: usize,
                };
                println!("{}", v[(p - 1 + current) % n]);
            }
            3 => {
                input! {
                    k: usize,
                };
                current = (current + k) % n;
            }
            _ => {}
        }
    }
}
