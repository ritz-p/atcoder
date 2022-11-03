use proconio::input;

fn main(){
    input!{
        n: usize,
    };
    let mut arr = Vec::new();
    for i in 1..{
        if i * i > n
        {
            break;
        }
        if n % i == 0
        {
            let x = n / i;
            arr.push(x);
            if i != x
            {
                arr.push(i);
            }
        }
    }
    for i in 0..arr.len(){
        println!("{}",arr[i]);
    }   
}