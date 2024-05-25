use proconio::input;
fn main(){
    input!{
        n: usize,
        mut lr: [(usize,usize);n]
    };
    let mut left = vec![];
    let mut right = vec![];
    for (l,r) in lr{
        left.push(l);
        right.push(r);
    }
    left.sort();
    right.sort();
    // 全部が重なっている範囲がある状態からスタート
    let mut res = n * (n - 1) / 2;
    let mut j = 0;
    for i in left{
        while right[j] < i{
            j += 1;
        }
        res -= j;
    }
    println!("{}",res);
}
