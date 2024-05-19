use proconio::input;

fn main(){
    input!{
        mut a: usize,
        b: usize,
        d: usize,
    };
    while a < b{
        print!("{} ",a);
        a += d;
    }
    print!("{}",b);
}
