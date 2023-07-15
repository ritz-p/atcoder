use proconio::input;

fn main(){
    input!{
        n: usize,
        p: usize,
        q: usize,
        arr: [usize;n]
    };
    let mut min = p;
    for i in 0..n{
        if min > q+arr[i] {
            min = q+arr[i];
        }
    }

    println!("{}",min);
}
