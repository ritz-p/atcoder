use proconio::input;

fn main(){
    input!{
        n: usize,
        mut arr: [usize;n],
    };
    let mut i = 0;
    while i < arr.len()-1{
        if arr[i] < arr[i+1]{
            if arr[i+1]-arr[i] != 1{
                let diff = arr[i+1]-arr[i];
                for j in 1..diff{
                    arr.insert(i+j,arr[i]+j);
                }
                i += diff-1;
            }
        }else if arr[i] > arr[i+1]{
            if arr[i]-arr[i+1] != 1{
                let diff = arr[i]-arr[i+1];
                for j in 1..diff{
                    arr.insert(i+j,arr[i]-j);
                }
                i += diff-1;
            }
        }
        i += 1;
    }
    for j in arr{
        print!("{} ",j);
    }
}
