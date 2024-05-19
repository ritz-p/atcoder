use proconio::input;

fn main(){
    input!{
        n: usize,
        g: [[usize;n];n]
    };

    for i in 0..n{
        for j in 0..n{
            if g[i][j] == 1{
                print!("{} ",j+1);
            }
        }
        println!();
    }
}
