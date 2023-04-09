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
    while count * count <= n{
        if m % count == 0{
            if !count_arr.contains_key(&count){
                count_arr.insert(count,1);
            }else{
                *count_arr.entry(count).or_insert(0) += 1;
            }
            m /= count;
        }else{
            count += 1;
        }
    }
    if m > 1{
        *count_arr.entry(m).or_insert(0) += 1; 
    }
    let mut arr = vec![];
    for (i,j) in &count_arr{
        let mut num = *i;
        let mut element_count = 1;
        while element_count < *j{
            num += *i;
            let mut num_cp = num;
            while num_cp > 1{
                if num_cp % *i == 0{
                    element_count+=1;
                    num_cp /= *i;
                }else{
                    break;
                }
            }
        }
        arr.push(num);
    }
    arr.sort_unstable();
    println!("{}",arr[arr.len()-1]);
}