use proconio::input;
    
fn main(){
    input!{
        mut a: usize,
        mut b: usize,
    };
    let mut res = 0;
    while a != b{
        if a < b{
            if b % a == 0{
                res += b / a - 1;
                break;
            }
            res += b / a;
            b = b % a;
        }else{
            if a % b == 0{
                res += a / b - 1;
                break;
            }
            res += a / b;
            a = a % b;
        }
    }
    println!("{}",res);
}
