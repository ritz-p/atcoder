use proconio::input;

fn main(){
    input!{
        n: usize,
        xy: [(usize,usize);n]
    };
    let mut x_sum = 0;
    let mut y_sum = 0;
    for (x,y) in xy{
        x_sum += x;
        y_sum += y;
    }
    if x_sum > y_sum{
        println!("Takahashi");
    }else if x_sum < y_sum{
        println!("Aoki");
    }else{
        println!("Draw");
    }
}
