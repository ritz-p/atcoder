use proconio::input;

fn main(){
    input!{
        n:usize,
        s:usize,
    };
    let mut res=0;
    for red in 1..=n{
        for blue in 1..=n{
            if red+blue<=s{
                res+=1;
            }
        }
    }
    println!("{}",res);
}