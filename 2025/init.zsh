#!/usr/bin/env zsh

local day=$1
[[ -n $day ]] || { echo '$1 is day'; return 1 }

local padded_day=$( python -c "print('$day'.zfill(2), end=None)" )

local mod_str="pub mod day$padded_day;"
local lib_file=src/lib.rs
rg -q $mod_str $lib_file || sed -ri "$( rg 'pub mod' $lib_file | wc -l ) a $mod_str" $lib_file
cat /dev/null > src/day${padded_day}.rs

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
