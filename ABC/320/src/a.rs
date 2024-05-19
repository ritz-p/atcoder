use proconio::input;

fn main(){
    input!{
        a: usize,
        b: usize
    };
    let mut a_res = 1;
    let mut b_res = 1;
    for i in 0..a{
        b_res *= b;
    }
    for i in 0..b{
        a_res *= a;
    }
    println!("{}",a_res+b_res);
}
