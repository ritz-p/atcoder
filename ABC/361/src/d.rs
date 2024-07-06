use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    let sb_count = s.iter().filter(|c|**c == 'B').count();
    let sw_count = s.iter().filter(|c| **c == 'W').count();
    let tb_count = t.iter().filter(|c|**c == 'B').count();
    let tw_count = t.iter().filter(|c| **c == 'W').count();

    if sb_count != tb_count || sw_count != tw_count{
        println!("-1");
        return;
    }

    let mut count = 0;
    for i in 0..n{
        if s[i] != t[i]{
            count += 1;
        }
    }

    println!("{}",count+1);
}