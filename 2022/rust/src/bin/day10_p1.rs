use rust::day10::get_operations;

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");

    let ops = get_operations("input/day10");

    let mut add_started = false;
    let mut cur_cycle = 1;
    let mut next_cycle = 20;
    let mut reg = 1;
    let mut op_idx = 0;

    let mut result = 0;

    loop {
        let op = &ops[op_idx];

        // Oh WOW, this is the worst!!! Avoiding implementing PartialEq for the enum!!!
        if op.code.to_int() == 2 {
            if add_started {
                add_started = false;
                op_idx += 1;
            } else {
                add_started = true;
            }
        } else {
            op_idx += 1;
        }

        if cur_cycle == next_cycle {
            // println!("during cycle {}, reg is {}", cur_cycle, reg);
            result += cur_cycle * reg;
            next_cycle += 40;
        }
        cur_cycle += 1;

        if !add_started {
            reg += op.value;
        }

        if op_idx == ops.len() && !add_started {
            break;
        }
    }

    println!("result: {}", result);

    println!("------------------- PART 1 -------------------");
}
