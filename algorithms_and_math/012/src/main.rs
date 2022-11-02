use proconio::input;

fn main(){
    input!{
        n: usize,
    };
    let mut res = false;
    if n == 2{
        res = true;
    }
    for i in 2..n{
        if n % i==0{
            break;
        }
        if i * i  >= n {
            res = true;
            break;
        }
    }
    if res{
        println!("Yes");
    }else{
        println!("No");
    }
}