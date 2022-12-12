use std::collections::{HashMap, HashSet};

use rust::day11::*;

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut fuck_map: HashMap<usize, Monkey> = HashMap::new();
    fuck_map.insert(
        1,
        Monkey {
            false_target_idx: 3,
            items: vec![79, 98],
            op: Op::Multiply(19),
            test_div: 23,
            true_traget_idx: 2,
        },
    );
    fuck_map.insert(
        2,
        Monkey {
            false_target_idx: 0,
            items: vec![54, 65, 75, 74],
            op: Op::Plus(6),
            test_div: 19,
            true_traget_idx: 2,
        },
    );

    fuck_map.insert(
        3,
        Monkey {
            false_target_idx: 3,
            items: vec![79, 60, 97],
            op: Op::Pow2,
            test_div: 13,
            true_traget_idx: 1,
        },
    );

    fuck_map.insert(
        4,
        Monkey {
            false_target_idx: 1,
            items: vec![74],
            op: Op::Pow2,
            test_div: 17,
            true_traget_idx: 0,
        },
    );

    monkeys.push(Monkey {
        false_target_idx: 3,
        items: vec![79, 98],
        op: Op::Multiply(19),
        test_div: 23,
        true_traget_idx: 2,
    });
    monkeys.push(Monkey {
        false_target_idx: 0,
        items: vec![54, 65, 75, 74],
        op: Op::Plus(6),
        test_div: 19,
        true_traget_idx: 2,
    });
    monkeys.push(Monkey {
        false_target_idx: 3,
        items: vec![79, 60, 97],
        op: Op::Pow2,
        test_div: 13,
        true_traget_idx: 1,
    });
    monkeys.push(Monkey {
        false_target_idx: 1,
        items: vec![74],
        op: Op::Pow2,
        test_div: 17,
        true_traget_idx: 0,
    });

    for i in 0..4 {
        let mut monkey = fuck_map.get(&i).unwrap();
        let mut res = monkey.do_actions();
        let false_idx = monkey.false_target_idx;
        let true_idx = monkey.true_traget_idx;

        let mut fuck1 = monkeys[false_idx].to_owned();
        fuck(&mut fuck1, &mut res.true_items);
        // *(monkeys)[false_idx].receive(&mut res.false_items);
        // monkeys[true_idx].receive(&mut res.true_items);

        // dbg!(&monkeys[false_idx]);
    }

    println!("------------------- PART 1 -------------------");
}

fn fuck(monkey: &mut Monkey, fuck2: &mut Vec<usize>) {
    monkey.receive(fuck2);
}
