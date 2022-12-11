use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Strategy {
    Win = 6,
    Loose = 0,
    Draw = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Only need to compare `Rock` and `Scissors` since their digit value order
        //  doesn't match how they're compared.
        if self == &Move::Rock && other == &Move::Scissors {
            Some(Ordering::Greater)
        } else if self == &Move::Scissors && other == &Move::Rock {
            Some(Ordering::Less)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(format!("unknown character '{}'", s).to_string()),
        }
    }
}

impl FromStr for Strategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Strategy::Loose),
            "Y" => Ok(Strategy::Draw),
            "Z" => Ok(Strategy::Win),
            _ => Err("well, that was unexpected!".to_string()),
        }
    }
}

fn get_next_move(cur_move: Move, strategy: Strategy) -> Move {
    let my_thing = match cur_move {
        Move::Paper => match strategy {
            Strategy::Win => Move::Scissors,
            Strategy::Loose => Move::Rock,
            Strategy::Draw => cur_move,
        },
        Move::Scissors => match strategy {
            Strategy::Win => Move::Rock,
            Strategy::Loose => Move::Paper,
            Strategy::Draw => cur_move,
        },
        Move::Rock => match strategy {
            Strategy::Win => Move::Paper,
            Strategy::Loose => Move::Scissors,
            Strategy::Draw => cur_move,
        },
    };

    // println!("{:?} --- {:?} ---> {:?}", cur_move, strategy, my_thing);
    my_thing
}

pub fn part1(input: &str) {
    println!("++++++++++ QUESTION 1 ++++++++++");

    let result: u16 = input
        .trim()
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .trim()
                .split(" ")
                .map(|c| c.parse::<Move>().unwrap())
                .collect();

            let my_move = moves[1];
            let their_move = moves[0];
            match their_move.partial_cmp(&my_move) {
                Some(Ordering::Less) => 6 + my_move as u16,
                Some(Ordering::Equal) => 3 + my_move as u16,
                Some(Ordering::Greater) => 0 + my_move as u16,
                _ => panic!("what happend? no partial cmp"),
            }
        })
        .sum();

    println!("{}", result);

    println!("---------- QUESTION 1 ----------");
}

pub fn part2(input: &str) {
    println!("++++++++++ QUESTION 2 ++++++++++");

    let result: u32 = input
        .trim()
        .lines()
        .map(|line| {
            let theirs = line
                .chars()
                .nth(0)
                .unwrap()
                .to_string()
                .parse::<Move>()
                .unwrap();

            let strategy = line
                .chars()
                .nth(2)
                .unwrap()
                .to_string()
                .parse::<Strategy>()
                .unwrap();

            let my_move = get_next_move(theirs, strategy);

            let move_point = my_move as u32;
            let strategy_point = strategy as u32;

            // println!(
            //     "move ({:?}) + strategy ({:?}) = {:?}",
            //     move_point,
            //     strategy_point,
            //     move_point + strategy_point
            // );

            move_point + strategy_point
        })
        .sum();

    println!("{}", result);

    println!("---------- QUESTION 2 ----------");
}
