alphabet=(A B C D E F G Ex)
for dir in $(ls ABC); do
    mkdir -p ABC/$dir/src
    touch ABC/$dir/Cargo.toml
    echo "[package]
name = \"abc_$dir\"
version = \"0.1.0\"
authors = [\"root\"]
edition = \"2018\"

[dependencies]
proconio = \"0.4.5\"
itertools = \"0.10.5\"" > ABC/$dir/Cargo.toml

    for abc in ${alphabet[@]}; do
        lower_case=$(echo $abc | tr 'A-Z' 'a-z')
        echo "
[[bin]]
name = \"$lower_case\"
path = \"src/$lower_case.rs\"" >> ABC/$dir/Cargo.toml
        mv ABC/$dir/$abc/src/main.rs ABC/$dir/src/$lower_case.rs
        rm -rf ABC/$dir/$abc/
    done
done