use proconio::input;
use std::cmp;

fn main() {
    input! {
        mut s: proconio::marker::Chars,
        mut t: proconio::marker::Chars
    }

    let mut memo: Vec<Vec<i32>> = vec![vec![0; t.len() + 1]; s.len() + 1];
    let mut flag_equal: i32 = 0;
    let mut answer: Vec<char> = Vec::new();
    let mut location_s: usize = s.len();
    let mut location_t: usize = t.len();
    for i in 1..s.len() + 1 {
        for j in 1..t.len() + 1 {
            flag_equal = 0;
            if s[i - 1] == t[j - 1] {
                flag_equal = 1;
            }
            memo[i][j] = cmp::max(memo[i - 1][j - 1] + flag_equal, cmp::max(memo[i][j - 1], memo[i - 1][j]));
        }
    }
    while memo[location_s][location_t] != 0 {
        if memo[location_s][location_t - 1] == memo[location_s][location_t] {
            location_t -= 1;
        }
        else if memo[location_s - 1][location_t] == memo[location_s][location_t] {
            location_s -= 1;
        }
        else if memo[location_s - 1][location_t - 1] + 1 == memo[location_s][location_t] {
            answer.push(s[location_s - 1]);
            location_s -= 1;
            location_t -= 1;
        }
    }
    if answer.len() == 0 {
        print!("{}", " ")
    }
    else {
        for chr in (0..answer.len()).rev() {
            print!("{}", answer[chr])
        }
    }
}
