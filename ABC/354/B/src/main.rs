use proconio::input;
fn main(){
    input!{
        n: usize,
        mut user: [(String,usize);n]
    };
    let mut sum = 0;
    for i in 0..n{
        sum += user[i].1;
    }

    user.sort_by(|a,b|a.0.cmp(&b.0));
    let res = &user[sum % n].0;
    println!("{}",res);
}
