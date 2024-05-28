use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }
    let mut ans = std::usize::MAX;
    for i in 0..10 {
        let mut cnt = vec![0; 10];
        for j in 0..n {
            let index = s[j].iter().position(|&si| si as u8 - b'0' == i).unwrap();
            cnt[index] += 1;
        }
        let cnt_max = *cnt.iter().max().unwrap();
        let r = cnt.iter().rposition(|&c| c == cnt_max).unwrap();
        let tmp = r + (cnt_max - 1) * 10;
        ans = ans.min(tmp);
    }
    println!("{ans}");
}
