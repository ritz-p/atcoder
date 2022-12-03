use proconio::input;
use std::collections::HashMap;
fn main(){
    input!{
        // n < 1000000000000
        n: usize,
    };
    let mut m = n;
    let mut count = 2;
    let mut count_arr:HashMap<usize, usize> = HashMap::new();

    let mut arr = vec![];
    while m > 1{
        if m % count == 0{
            if !count_arr.contains_key(&count){
                arr.insert(count,1);
            }
            *count_arr.entry(count).or_insert(0) += 1;
            // count_arr.insert(count,insert);
            m /= count;
        }else{
            count += 1;
        }
    }
    // let mut arr2 = vec![];
    // for i in 0..1000000000000{
    //     if count_arr[i] > 0{
    //         let mut num = i;
    //         for _j in 0..count_arr[i]{
    //             num *= i;
    //         }
    //         arr2.push(num);
    //     }
    // }
    // arr2.sort_unstable();
    // println!("{}",arr[arr2.len()-1]);
}
