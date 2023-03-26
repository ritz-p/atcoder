use proconio::input;
    
fn main(){
    input!{
        s: String,
    };
    let mut flag = 0;
    let mut bk = vec![0_u64;1 << 10];
    bk[0] = 1;
    let mut res = 0;
    for c in s.chars(){
        flag ^= 1 << c as usize - '0' as usize;
        res += bk[flag as usize];
        bk[flag as usize] += 1;
    }
    println!("{}",res);
}