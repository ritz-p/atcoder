use proconio::input;

fn main(){
    input!{
        n:usize,
    };
    let mut arr = vec![false;n+1];

    for i in 2..=n{
        for j in 2..=i{
            if i != j && i % j == 0{
                break;
            }
            if i/2 <= j{
                arr[i] = true;
            }
        }
    }

    for i in 2..=n{
        if arr[i]{
            print!("{} ",i)
        }
    }
}
