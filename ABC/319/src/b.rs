use proconio::input;

fn main(){
    input!{
        n: usize
    };
    let mut s = "".to_string();

    for i in 0..=n{
        let mut is_minus = true;
        for j in 1..=9{
            if n % j == 0{
                let k = n / j;
                if i % k == 0{
                    s += &j.to_string();
                    is_minus = false;
                    break;
                }
            }
        }
        if is_minus{
            s += "-";
        }
    }
    println!("{}",s);
}
