mod items;

use std::time::Instant;

use items::{Item, ITEM_SETS};

const START_N: usize = 10;
const MAX_N: usize = 50;


fn main() {
    println!("Hello, world!");

    for n in START_N..=MAX_N {
        for i in 0..3 {
            let items = ITEM_SETS[i].to_vec();

            let start = Instant::now();

            // let (solution, solution_weight, solution_value) = brgc_knapsack(&items, n);
            let test_value = 21;

            let duration = start.elapsed();

            // write_to_file(
            //     &solution,
            //     &solution_weight,
            //     &solution_value,
            //     &duration,
            //     &i,
            //     &n,
            // );
        }
    }
}

fn dp_tab_solve(items: &Vec<Item>, n: usize) {


}

fn dp_mem_solve(items: &Vec<Item>, n: usize) {

}

// todo: include way to pass in different evaluation/heuristic functions
fn greedy_solve(items: &Vec<Item>, n: usize, h: fn(&Item) -> f32) {

}
