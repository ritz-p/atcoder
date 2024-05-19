if [ -z "${1}" ]; then
    echo "You should set contest number"
    exit 1
fi

echo "[workspace]
members = [
    \"ABC/$1\",
    \"ironclad_rule\"
]" > Cargo.toml;