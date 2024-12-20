pub mod item;
pub mod items;
pub mod recorder;

use item::Item;
use items::generate_items;
use recorder::{IterationResult, Record, Recorder};
use std::error::Error;
use std::thread;
use std::time::Instant;

const START_N: usize = 100;
const MAX_N: usize = 100_000;
const STEP: usize = 100;
const KNAPSACK_CAPACITY: usize = 1000;

fn main() -> Result<(), Box<dyn Error>> {
    let mut recorder = Recorder::new("results/results.csv");
    recorder.default_setup()?;

    for n in (START_N..=MAX_N).step_by(STEP) {
        let mut iteration_result: IterationResult = IterationResult::new(n);

        let item_sets = generate_items();

        for i in 0..3 {
            let items = item_sets[i][0..n].to_vec();

            // DP Memoization
            run_trial(
                &items,
                &mut iteration_result.dp_mem,
                false,
                |items, capacity| dp_mem_solve(items, capacity),
            );

            // DP Tabulation
            run_trial(
                &items,
                &mut iteration_result.dp_tab,
                true,
                |items, capacity| dp_tab_solve(items, capacity),
            );

            // Greedy Highest Value
            run_trial(
                &items,
                &mut iteration_result.greedy_highest_value,
                true,
                |items, capacity| {
                    let compare = |a: &Item, b: &Item| b.value.cmp(&a.value);
                    greedy_solve(items, capacity, compare)
                },
            );

            // Greedy Smallest Weight
            run_trial(
                &items,
                &mut iteration_result.greedy_smallest_weight,
                true,
                |items, capacity| {
                    let compare = |a: &Item, b: &Item| a.weight.cmp(&b.weight);
                    greedy_solve(items, capacity, compare)
                },
            );

            // Greedy Greatest Ratio
            run_trial(
                &items,
                &mut iteration_result.greedy_greatest_ratio,
                true,
                |items, capacity| {
                    let compare =
                        |a: &Item, b: &Item| (b.value / b.weight).cmp(&(a.value / a.weight));
                    greedy_solve(items, capacity, compare)
                },
            );
        }

        recorder.write_iteration_result(iteration_result)?;

        println!("Finished n = {}", n);
    }
    Ok(())
}

fn run_trial(
    items: &Vec<Item>,
    results: &mut Vec<Record>,
    same_thread: bool,
    algorithm: fn(Vec<Item>, usize) -> i32,
) {
    let items_copy = items.clone();

    if same_thread {
        let start = Instant::now();
        let solution_value = algorithm(items_copy, KNAPSACK_CAPACITY);
        let duration = start.elapsed();

        results.push(Record::new(solution_value as usize, duration.as_micros()));
    } else {
        // 0.25 KB per possible recursion call ¯\_(ツ)_/¯
        // I don't know if that's actually enough for a call,
        // so this is taking advantage of the fact that the algo
        // is unlikely to actually reach n * capacity calls
        let stack_size = items.len() * (KNAPSACK_CAPACITY) * 256;
        let (solution_value, duration) = thread::Builder::new()
            .stack_size(stack_size)
            .spawn(move || {
                let start = Instant::now();
                let solution_value = algorithm(items_copy, KNAPSACK_CAPACITY);
                let duration = start.elapsed();

                (solution_value, duration)
            })
            .unwrap()
            .join()
            .unwrap();

        results.push(Record::new(solution_value as usize, duration.as_micros()));
    }
}

// https://www.geeksforgeeks.org/0-1-knapsack-problem-dp-10/#tabulation-or-bottomup-approach-on-x-w-time-and-space
fn dp_tab_solve(items: Vec<Item>, capacity: usize) -> i32 {
    let n: usize = items.len();
    let mut dp = vec![vec![0; capacity + 1]; n + 1];

    for i in 1..=n {
        for w in 1..=capacity {
            dp[i][w] = dp[i - 1][w];
            if items[i - 1].weight <= w.try_into().unwrap() {
                dp[i][w] = std::cmp::max(
                    dp[i - 1][w],
                    items[i - 1].value + dp[i - 1][w - (items[i - 1].weight as usize)],
                );
            } else {
                dp[i][w] = dp[i - 1][w];
            }
        }
    }

    return dp[n][capacity];
}

// https://www.geeksforgeeks.org/0-1-knapsack-problem-dp-10/#memoization-approach-on-x-w-time-and-space
fn dp_mem_solve(items: Vec<Item>, capacity: usize) -> i32 {
    let n = items.len() as isize;
    let mut memo = vec![vec![-1; capacity + 1]; n as usize];

    return dp_mem_rec(&items, capacity, n - 1, &mut memo);
}

fn dp_mem_rec(items: &Vec<Item>, capacity: usize, index: isize, memo: &mut Vec<Vec<i32>>) -> i32 {
    if index < 0 {
        return 0;
    }

    if memo[index as usize][capacity] != -1 {
        return memo[index as usize][capacity];
    }

    let result = if items[index as usize].weight as usize > capacity {
        dp_mem_rec(items, capacity, index - 1, memo)
    } else {
        std::cmp::max(
            dp_mem_rec(items, capacity, index - 1, memo),
            items[index as usize].value
                + dp_mem_rec(
                    items,
                    capacity - items[index as usize].weight as usize,
                    index - 1,
                    memo,
                ),
        )
    };
    memo[index as usize][capacity] = result;

    return result;
}

fn greedy_solve(
    mut items: Vec<Item>,
    capacity: usize,
    compare: fn(&Item, &Item) -> std::cmp::Ordering,
) -> i32 {
    let capacity = capacity as i32;

    items.sort_by(compare);

    let mut total_weight = 0;
    let mut total_value = 0;

    for item in items {
        if total_weight + item.weight <= capacity {
            total_weight += item.weight;
            total_value += item.value;
        }
    }

    return total_value as i32;
}
