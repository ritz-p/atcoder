use proconio::input;

const MOD: i64 = 998244353;

fn main() {
    input! {
        n: usize
    };

    let res = (0..(n - 1))
        .fold([1, 1, 1, 1, 1, 1, 1, 1, 1], |acc, _| {
            let mut next = acc.clone();
            for i in 0..9 {
                if i > 0 {
                    next[i] += acc[i - 1];
                }
                if i < 8 {
                    next[i] += acc[i + 1];
                }
                next[i] %= MOD;
            }
            next
        })
        .iter()
        .sum::<i64>()
        % MOD;
    println!("{}", res);
}
