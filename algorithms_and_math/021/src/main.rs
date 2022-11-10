use proconio::input;

fn main(){
    input!{
        n: usize,
        r: usize,
    };
    let res = ncr(n,r);
    println!("{}",res);
}

fn ncr(n:usize,r:usize)->usize{
    let mut min = 1;
    if n == r{
        return 1
    }
    if n-r >= r{
        min = r;
    }else if n-r < r{
        min = n-r;
    }
    let mut n2 = 1;
    let mut r2 = 1;
    for i in 0..min{
        n2 *= n - i;
    }
    for i in 1..=min{
        r2 *= i;
    }
    n2/r2
}

// fn ncr(n:usize,r:usize)->usize{
//     let mut min = 0;
//     if n == r{
//         return 1
//     }
//     if n-r >= r{
//         min = r;
//     }else if n-r < r{
//         min = n-r;
//     }
//     // println!("{}",min);
//     let mut r_arr = vec![0;min-1];
//     let mut n_arr = vec![0;min];
//     for i in 0..min-1{
//         r_arr[i] = min-i;
//         // println!("{}",r_arr[i]);
//     }
//     for i in 0..min{
//         n_arr[i] = n-i;
//         // println!("{}",n_arr[i]);
//     }
//     for i in 0..min{
//         for j in 0..min-1{
//             if r_arr[j] != 1 && n_arr[i] % r_arr[j] == 0{
//                 n_arr[i] /= r_arr[j];
//                 r_arr[j] = 1;
//                 println!("{}",n_arr[i]);
//             }
//         }
//     }
//     let mut res = 1;
//     for i in 0..min{
//         res *= n_arr[i];
//         // println!("{}",res);
//     }
//     res
// }