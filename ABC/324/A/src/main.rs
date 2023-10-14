use proconio::input;

fn main(){
    input!{
        n: usize,
        a: [usize;n]
    };
    for i in 1..n{
        if a[0] != a[i]{
            println!("No");
            return
        }
    }
    println!("Yes");
}
