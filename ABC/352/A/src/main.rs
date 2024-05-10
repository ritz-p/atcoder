use proconio::input;

fn main(){
    input!{
        _n: usize,
        x: usize,
        y: usize,
        z: usize,
    };
    if x < y{
        if x < z && z < y{
            println!("Yes");
        }else{
            println!("No");
        }
    }else{
        if y < z && z < x{
            println!("Yes");
        }else{
            println!("No");
        }
    }

}
