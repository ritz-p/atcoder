use proconio::input;

fn main() {
    input!{
        n: usize,
        k: usize,
        s: String
    }
    let mut count = 0;
    for i in 0..n{
        if s.chars().nth(i).unwrap() == 'o'{
            if count < k{
                count += 1;
                print!("{}",s.chars().nth(i).unwrap());
            }else{
                print!("x");
            }
        }else{
            print!("{}",s.chars().nth(i).unwrap());
        }
    }
}
