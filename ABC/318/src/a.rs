use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        p: usize
    };
    if n < m{
        println!("{}",0);
        return;
    }
    let l = n - m;
    let res = l / p;
    println!("{}",res+1);

}
