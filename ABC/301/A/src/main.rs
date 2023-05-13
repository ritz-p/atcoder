use proconio::input;

fn main(){
    input!{
        n: usize,
        s: String,
    };
    let mut a = 0;
    let mut t = 0;

    for i in s.chars(){
        if i == 'T'{
            t += 1;
        }else if i == 'A'{
            a += 1;
        }
        if n % 2 == 0{
            if a >= n/2{
                println!("A");
                break;
            }else if t >= n/2{
                println!("T");
                break;
            }
        }else if n % 2 == 1{
            if a > n/2{
                println!("A");
                break;
            }else if t > n/2{
                println!("T");
                break;
            }
        }
    }
}
