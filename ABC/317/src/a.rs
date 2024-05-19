use proconio::input;

fn main(){
    input!{
        n: usize,
        h: usize,
        x: usize,
        p: [usize;n]
    };
    let mut min = 1000;
    let mut pos = 0;
    for i in 0..n{
        if h + p[i] >= x{
            if min > p[i]{
                min = p[i];
                pos = i;
            }
        }
    }
    println!("{}",pos+1);
}
