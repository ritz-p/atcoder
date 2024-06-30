use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize,usize);m]
    };
    let mut friends = vec![vec![]; n];
    for (a, b) in ab {
        friends[a - 1].push(b);
        friends[b - 1].push(a);
    }
    let mut m = 0;
    let mut current = 0;
    for i in 0..n {
        if friends[i].len() >= m {
            m = friends[i].len();
            current = i;
        }
    }
    println!("{}", current + 1);
}
