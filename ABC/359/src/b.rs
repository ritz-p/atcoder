use proconio::input;

fn main(){
    input!{
        n: usize,
        a: [usize;2*n]
    };
    let mut res = 0;
    for i in 1..2*n-1{
        if a[i-1] == a[i+1]{
            res += 1;
        }
    }

    println!("{}",res);
}
