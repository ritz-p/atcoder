use proconio::input;

fn main(){
    input!{
        h: usize,
        w: usize,
        n: usize,
    };
    let mut grid:Vec<Vec<char>> = vec![vec!['.';w];h];
    let mut direction = 0;
    let mut current_i = 0;
    let mut current_j = 0;
    for _i in 0..n{
        if grid[current_i][current_j] == '.'{
            grid[current_i][current_j] = '#';
            if direction == 0{
                direction = 1;
                if current_j == w-1{
                    current_j = 0;
                }else{
                    current_j += 1;
                }
            }else if direction == 1{
                direction = 2;
                if current_i == h-1{
                    current_i = 0;
                }else{
                    current_i += 1;
                }
            }else if direction == 2{
                direction = 3;
                if current_j == 0{
                    current_j = w-1;
                }else{
                    current_j -= 1;
                }
            }else if direction == 3{
                direction = 0;
                if current_i == 0{
                    current_i = h-1;
                }else{
                    current_i -= 1;
                }
            }
        }else if grid[current_i][current_j] == '#'{
            grid[current_i][current_j] = '.';
            if direction == 0{
                direction = 3;
                if current_j == 0{
                    current_j = w-1;
                }else{
                    current_j -= 1;
                }
            }else if direction == 1{
                direction = 0;
                if current_i == 0{
                    current_i = h-1;
                }else{
                    current_i -= 1;
                }
            }else if direction == 2{
                direction = 1;
                if current_j == w-1{
                    current_j = 0;
                }else{
                    current_j += 1;
                }
            }else if direction == 3{
                direction = 2;
                if current_i == h-1{
                    current_i = 0;
                }else{
                    current_i += 1;
                }
            }
        }
    }
    for i in 0..h{
        for j in 0..w{
            print!("{}",grid[i][j]);
        }
        println!();
    }
}
