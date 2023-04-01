use proconio::input;
    
fn main(){
    input!{
        n: usize,
        s: String
    };
    for i in 0..n-1{
        if s.chars().nth(i).unwrap() == 'M'{
            if s.chars().nth(i+1).unwrap() == 'M'{
                println!("No");
                return
            }
        }else if s.chars().nth(i).unwrap() == 'F'{
            if s.chars().nth(i+1).unwrap() == 'F'{
                println!("No");
                return
            }
        }
    }
    println!("Yes");
}
