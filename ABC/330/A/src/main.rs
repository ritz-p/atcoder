use proconio::input;

fn main(){
    input!{
        n: usize,
        l: usize,
        a: [usize;n]
    };
    let mut res = 0;
    for i in a{
        if i >= l{
            res += 1;
        }
    }
    println!("{}",res);
}
