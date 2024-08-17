use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize;n]
    };

    let mut distance = vec![0; 2 * n];
    for i in 1..2 * n {
        distance[i] = (distance[i - 1] + a[(i - 1) % n]) % m;
    }

    let mut table = vec![0; m as usize];
    for i in 0..n {
        table[distance[i] as usize] += 1;
    }

    let mut count: i64 = 0;
    for i in 0..n {
        table[distance[i] as usize] -= 1;
        count += table[distance[i] as usize];
        table[(distance[n + i]) as usize] += 1;
    }

    println!("{}", count);
}
