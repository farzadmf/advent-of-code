#[derive(Debug, Copy, Clone)]
pub enum Op {
    Plus(usize),
    Multiply(usize),
    Pow2,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub false_target_idx: usize,
    pub items: Vec<usize>,
    pub op: Op,
    pub test_div: usize,
    pub true_traget_idx: usize,
}

#[derive(Debug)]
pub struct MoveItems {
    pub true_items: Vec<usize>,
    pub false_items: Vec<usize>,
}

impl Monkey {
    pub fn receive(&mut self, items: &mut Vec<usize>) {
        self.items.append(items);
    }

    pub fn do_actions(&mut self) -> MoveItems {
        let mut true_items: Vec<usize> = vec![];
        let mut false_items: Vec<usize> = vec![];

        for idx in 0..self.items.len() {
            let mut worry_level = self.items[idx];

            match self.op {
                Op::Plus(val) => worry_level += val,
                Op::Multiply(val) => worry_level *= val,
                Op::Pow2 => worry_level *= worry_level,
            }

            worry_level = (worry_level as f32 / 3.0).floor() as usize;
            if worry_level % self.test_div == 0 {
                true_items.push(worry_level);
            } else {
                false_items.push(worry_level)
            }
        }

        self.items.clear();

        MoveItems {
            true_items,
            false_items,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Item {
    pub worry_level: usize,
}
