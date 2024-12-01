#!/usr/bin/env zsh

local input=./input/day01

local col1=($(cat $input | awk '{print $1}'))
local col2=($(cat $input | awk '{print $2}'))

local col1_sort=($(printf '%s\n' $col1 | sort -n))
local col2_sort=($(printf '%s\n' $col2 | sort -n))

local count=${#col1_sort}

local total=0
for ((i = 1; i <= $count; i++)); do
  local diff=$((${col1_sort[i]} - ${col2_sort[i]}))
  local diff=$((diff < 0 ? -diff : diff))
  ((total += diff))
done

echo "total (part 1) = $total"
echo '---------------'

typeset -Ax dict
typeset -Ax seen
local total2=0
for ((i = 1; i <= $count; i++)); do
  local c1=${col1[i]}

  ((seen[$c1] == 1)) || {
    seen[$c1]=1

    for ((j = 1; j <= $count; j++)); do
      local c2=${col2[j]}
      ((c2 == c1)) && dict[$c1]=$((${dict[$c1]:-0} + 1))
    done
  }

  # echo "for $c1, we have ${dict[$c1]:-0} count"
  ((total2 += ($c1 * ${dict[$c1]:-0})))
  # echo "total2 = $total2"
done

echo "total (part 2) = $total2"
