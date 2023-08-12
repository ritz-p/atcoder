use proconio::input;

fn main(){
    input!{
        n: usize
    }
    let pi:Vec<char> = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679".chars().collect();
    print!("{}{}",pi[0],pi[1]);
    for i in 0..n{
        print!("{}",pi[i+2]);
    }
}
