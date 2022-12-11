#!/usr/bin/env zsh

local day=$1
[[ -n $day ]] || { echo '$1 is day'; return 1 }

local padded_day=$( python -c "print('$day'.zfill(2), end=None)" )

rg -q $padded_day src/lib.rs || echo "pub mod day${padded_day};" >> src/lib.rs
cat /dev/null > $folder/src/day${padded_day}.rs

for p in {1..2}; do
  cat <<EOF >src/bin/day${padded_day}_p${p}.rs
fn main() {
    println!("+++++++++++++++++++ PART $p +++++++++++++++++++");
    let result = 0;

    // Let's do this!

    println!("result: {}", result);
    println!("------------------- PART $p -------------------");
}
EOF
done
