use std::collections::{BTreeMap, HashMap, HashSet};

use rust::day11::*;

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");

    let mut monkeys: Vec<Monkey> = Vec::new();
    monkeys.push(Monkey {
        false_target_idx: 3,
        inspect_count: 0,
        items: vec![79, 98],
        op: Op::Multiply(19),
        test_div: 23,
        true_traget_idx: 2,
    });
    monkeys.push(Monkey {
        false_target_idx: 0,
        inspect_count: 0,
        items: vec![54, 65, 75, 74],
        op: Op::Plus(6),
        test_div: 19,
        true_traget_idx: 2,
    });
    monkeys.push(Monkey {
        false_target_idx: 3,
        inspect_count: 0,
        items: vec![79, 60, 97],
        op: Op::Pow2,
        test_div: 13,
        true_traget_idx: 1,
    });
    monkeys.push(Monkey {
        false_target_idx: 1,
        inspect_count: 0,
        items: vec![74],
        op: Op::Plus(3),
        test_div: 17,
        true_traget_idx: 0,
    });

    for r in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            let mut res = monkey.inspect();
            let false_idx = monkey.false_target_idx;
            let true_idx = monkey.true_traget_idx;

            monkeys[false_idx].receive(&mut res.false_items);
            monkeys[true_idx].receive(&mut res.true_items);
        }

        // println!("============================ round {}", r + 1);
        // for i in 0..monkeys.len() {
        //     println!("monkey {i} items: {:?}", monkeys[i].items);
        // }
    }

    // for i in 0..monkeys.len() {
    //     println!("monkey {i} inspected {} items", monkeys[i].inspect_count);
    // }

    let mut counts = monkeys
        .into_iter()
        .map(|m| m.inspect_count)
        .collect::<Vec<_>>();

    counts.sort();

    println!(
        "multiply of 2 higher ones: {}",
        counts.iter().rev().take(2).product::<usize>()
    );

    println!("------------------- PART 1 -------------------");
}
