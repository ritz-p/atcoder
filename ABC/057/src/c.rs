use proconio::input;

fn main(){
    input!{
        n: usize
    };

    let mut index = 1;
    let mut res = usize::MAX;
    while index * index <= n{
        if n % index == 0{
            res = res.min(index.to_string().len().max((n/index).to_string().len()))
        }
        index += 1;
    }

    println!("{}",res);
}
