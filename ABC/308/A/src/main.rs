use proconio::input;

fn main(){
    input!{
        arr: [usize;8]
    };
    let mut res = true;
    for i in 0..8{
        if i < 7{
            if arr[i] > arr[i+1]{
                res = false;
                break;
            }
        }
        if arr[i] < 100 || arr[i] > 675{
            res = false;
            break;
        }
        if arr[i] % 25 != 0{
            res = false;
            break;
        }
    }

    if res{
        println!("Yes");
    }else{
        println!("No")
    }
}
