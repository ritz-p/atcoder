use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize;n]
    };
    let mut res = vec![0; n];
    let mut s = Vec::new();

    for i in (0..n - 1).rev() {
        while !s.is_empty() && h[*s.last().unwrap()] < h[i + 1] {
            s.pop();
        }
        s.push(i + 1);
        res[i] = s.len();
    }

    for a in res {
        print!("{} ", a);
    }
    println!();
}
