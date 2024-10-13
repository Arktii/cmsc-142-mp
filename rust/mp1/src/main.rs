use std::env;
use std::time::Instant;

const KNAPSACK_CAPACITY: i32 = 1000;

fn main() {
    // Get command-line arguments and parse n
    let args: Vec<String> = env::args().collect();
    let n: usize = args[1]
        .parse()
        .expect("Please provide n as a command-line argument e.g. cargo run -- 25");

    if n > items().len() {
        panic!(
            "n cannot be greater than the number of items ({}).",
            items().len()
        );
    }

    let start = Instant::now();

    let items = items();

    let (solution, solution_weight, solution_value) = brgc_knapsack(&items, n);

    let duration = start.elapsed();

    print!("Solution: ");
    print_solution(&solution);
    println!();
    println!("Weight: {}", solution_weight);
    println!("Value: {}", solution_value);

    println!("Total time: {:?}", duration);
}

fn brgc_knapsack(items: &Vec<(i32, i32)>, n: usize) -> (Vec<bool>, i32, i32) {
    let mut solution = vec![false; n];
    let mut solution_weight = 0;
    let mut solution_value = 0;

    let mut current = vec![false; n];
    let mut current_weight = 0;
    let mut current_value = 0;

    for i in 1..(1 << n) {
        let change_index = get_index_to_flip(&i);

        current[change_index] = !current[change_index];

        if current[change_index] {
            current_weight += items[change_index].0;
            current_value += items[change_index].1;
        } else {
            current_weight -= items[change_index].0;
            current_value -= items[change_index].1;
        }

        if current_weight <= KNAPSACK_CAPACITY && current_value > solution_value {
            solution = current.clone();
            solution_weight = current_weight;
            solution_value = current_value;
        }
    }

    (solution, solution_weight, solution_value)
}

fn get_index_to_flip(i: &i32) -> usize {
    i.trailing_zeros() as usize
}

fn print_solution(solution: &Vec<bool>) {
    print!("[");
    for i in 0..solution.len() - 1 {
        print!("{}", if solution[i] { 1 } else { 0 });
        print!(", ");
    }
    print!("{}", if solution[solution.len() - 1] { 1 } else { 0 });
    print!("]");
}

// Items pregenerated in Python
fn items() -> Vec<(i32, i32)> {
    vec![
        (51, 212),
        (50, 402),
        (62, 395),
        (59, 150),
        (64, 289),
        (65, 373),
        (68, 267),
        (92, 376),
        (79, 173),
        (68, 451),
        (59, 135),
        (96, 355),
        (96, 306),
        (80, 247),
        (55, 179),
        (71, 493),
        (81, 286),
        (88, 124),
        (82, 414),
        (74, 235),
        (87, 342),
        (68, 125),
        (96, 262),
        (84, 347),
        (76, 118),
        (82, 232),
        (91, 296),
        (84, 406),
        (50, 175),
        (61, 371),
        (50, 420),
        (89, 424),
        (97, 407),
        (63, 492),
        (79, 313),
        (85, 443),
        (66, 416),
        (89, 199),
        (85, 139),
        (94, 271),
        (97, 352),
        (93, 460),
        (69, 112),
        (55, 334),
        (72, 471),
        (53, 157),
        (96, 408),
        (90, 433),
        (60, 432),
        (70, 197),
    ]
}
