use proconio::*;
use std::collections::*;

const MOD: u64 = 998244353;

fn main() {
    let mut mods = vec![1];
    for _ in 0..600005 {
        mods.push((mods.last().unwrap() * 10) % MOD);
    }

    let mut s = VecDeque::new();
    s.push_back(1);
    let mut m = 1;

    input! {
        q: usize,
    }

    for _ in 0..q {
        input! { command: u8 }

        match command {
            1 => {
                input! {x: u8}
                s.push_back(x);
                m = (m * 10 + x as u64) % MOD;
            },
            2 => {
                let x = s.pop_front().unwrap();
                m = (m + MOD - (x as u64) * mods[s.len()] % MOD) % MOD;
            },
            _ => {
                println!("{}", m);
            }
        }
    }
}
