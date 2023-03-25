use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        r: usize,
        c: usize,
        mut graph: [Chars;r],
    };
    for i in 0..r{
        for j in 0..c{
            if graph[i][j] == '1'{
                graph[i][j] = '.';
                for k in 0..r{
                    for l in 0..c{
                        if ((i as f64 -k as f64).abs()+(j as f64 -l as f64).abs()) <= 1.0 && graph[k][l] == '#'{
                            graph[k][l] = '.';
                        }
                    }
                }
            }else if graph[i][j] == '2'{
                graph[i][j] = '.';
                for k in 0..r{
                    for l in 0..c{
                        if ((i as f64 -k as f64).abs()+(j as f64 -l as f64).abs()) <= 2.0 && graph[k][l] == '#'{
                            graph[k][l] = '.';
                        }
                    }
                }
            }else if graph[i][j] == '3'{
                graph[i][j] = '.';
                for k in 0..r{
                    for l in 0..c{
                        if ((i as f64 -k as f64).abs()+(j as f64 -l as f64).abs()) <= 3.0 && graph[k][l] == '#'{
                            graph[k][l] = '.';
                        }
                    }
                }
            }else if graph[i][j] == '4'{
                graph[i][j] = '.';
                for k in 0..r{
                    for l in 0..c{
                        if ((i as f64 -k as f64).abs()+(j as f64 -l as f64).abs()) <= 4.0 && graph[k][l] == '#'{
                            graph[k][l] = '.';
                        }
                    }
                }
            }else if graph[i][j] == '5'{
                graph[i][j] = '.';
                for k in 0..r{
                    for l in 0..c{
                        if ((i as f64 -k as f64).abs()+(j as f64 -l as f64).abs()) <= 5.0 && graph[k][l] == '#'{
                            graph[k][l] = '.';
                        }
                    }
                }
            }else if graph[i][j] == '6'{
                graph[i][j] = '.';
                for k in 0..r{
                    for l in 0..c{
                        if ((i as f64 -k as f64).abs()+(j as f64 -l as f64).abs()) <= 6.0 && graph[k][l] == '#'{
                            graph[k][l] = '.';
                        }
                    }
                }
            }else if graph[i][j] == '7'{
                graph[i][j] = '.';
                for k in 0..r{
                    for l in 0..c{
                        if ((i as f64 -k as f64).abs()+(j as f64 -l as f64).abs()) <= 7.0 && graph[k][l] == '#'{
                            graph[k][l] = '.';
                        }
                    }
                }
            }else if graph[i][j] == '8'{
                graph[i][j] = '.';
                for k in 0..r{
                    for l in 0..c{
                        if ((i as f64 -k as f64).abs()+(j as f64 -l as f64).abs()) <= 8.0 && graph[k][l] == '#'{
                            graph[k][l] = '.';
                        }
                    }
                }
            }else if graph[i][j] == '9'{
                graph[i][j] = '.';
                for k in 0..r{
                    for l in 0..c{
                        if ((i as f64 - k as f64).abs()+(j as f64 - l as f64).abs()) <= 9.0 && graph[k][l] == '#'{
                            graph[k][l] = '.';
                        }
                    }
                }
            }
        }
    }
    for i in 0..r{
        for j in 0..c{
            print!("{}",graph[i][j]);
        }
        print!("\n");
    }
}
