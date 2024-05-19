use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        mut a: [Usize1; n]
    };
    let c: usize = 2usize*10usize.pow(5)+1;
    for _ in 0..30 {
        a = a.iter().map(|&x| a[x]).collect::<Vec<usize>>();
    }
    let mut visited = vec![false; c];
    let mut answer = 0usize;
    for &a in a.iter() {
        if visited[a] {
            continue;
        }
        visited[a] = true;
        answer += 1;
    }
    println!("{}", answer)
}
