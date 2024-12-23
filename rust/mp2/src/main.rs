pub mod item;
pub mod items;
pub mod recorder;

use item::Item;
use items::generate_items;
use recorder::{IterationResult, NontrivialIterationResult, Record, Recorder};
use std::env;
use std::error::Error;
use std::thread;
use std::time::Instant;

const START_N: usize = 100;
const MAX_N: usize = 100_000;
const STEP: usize = 100;
const KNAPSACK_CAPACITY: usize = 1000;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let is_recording_count = if args.len() > 1 {
        args[1] == "record"
    } else {
        false
    };

    let mut recorder = Recorder::new("results/results.csv");
    let mut nontrivial_recorder = Recorder::new("results/nontrivial_entries_count.csv");

    if !is_recording_count {
        recorder.default_setup()?;
    } else {
        nontrivial_recorder.nontrivial_count_setup()?;
    }

    for n in (START_N..=MAX_N).step_by(STEP) {
        let mut iteration_result: IterationResult = IterationResult::new(n);
        let mut iteration_result_nontrivial: NontrivialIterationResult =
            NontrivialIterationResult::new(n);

        let item_sets = generate_items();

        for i in 0..3 {
            let items = item_sets[i][0..n].to_vec();

            if !is_recording_count {
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
            } else {
                // this records the nontrivial entries (computations and retrievals)
                // recheck if the computations and retrievals are counted correctly in dp_mem_count and dp_tab_count
                // haha

                // DP Memoization
                run_trial_2(
                    &items,
                    &mut iteration_result_nontrivial.dp_mem,
                    false,
                    |items, capacity| dp_mem_count(items, capacity),
                );

                // DP Tabulation
                run_trial_2(
                    &items,
                    &mut iteration_result_nontrivial.dp_tab,
                    true,
                    |items, capacity| dp_tab_count(items, capacity),
                );
            }
        }

        if !is_recording_count {
            recorder.write_iteration_result(iteration_result)?;
        } else {
            nontrivial_recorder.write_nontrivial_iteration_result(iteration_result_nontrivial)?;
        }

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
        // 256 bytes per possible recursion call ¯\_(ツ)_/¯
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
fn run_trial_2(
    items: &Vec<Item>,
    results: &mut Vec<(i32, i32)>,
    same_thread: bool,
    algorithm: fn(Vec<Item>, usize) -> (i32, i32),
) {
    let items_copy = items.clone();

    if same_thread {
        let data = algorithm(items_copy, KNAPSACK_CAPACITY);

        results.push(data);
    } else {
        // 256 bytes per possible recursion call
        let stack_size = items.len() * (KNAPSACK_CAPACITY) * 256;
        let data = thread::Builder::new()
            .stack_size(stack_size)
            .spawn(move || algorithm(items_copy, KNAPSACK_CAPACITY))
            .unwrap()
            .join()
            .unwrap();

        results.push(data);
    }
}

// https://www.geeksforgeeks.org/0-1-knapsack-problem-dp-10/#tabulation-or-bottomup-approach-on-x-w-time-and-space
fn dp_tab_solve(items: Vec<Item>, capacity: usize) -> i32 {
    let n: usize = items.len();
    let mut dp = vec![vec![0; capacity + 1]; n + 1];

    for i in 1..=n {
        for w in 1..=capacity {
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

// DP COUNTING NON-TRIVIAL COMPUTATIONS
fn dp_tab_count(items: Vec<Item>, capacity: usize) -> (i32, i32) {
    let n: usize = items.len();
    let mut dp = vec![vec![0; capacity + 1]; n + 1];
    let mut computed = 0;
    let mut retrieved = 0;

    for i in 1..=n {
        for w in 1..=capacity {
            computed += 1;
            if items[i - 1].weight <= w.try_into().unwrap() {
                dp[i][w] = std::cmp::max(
                    dp[i - 1][w],
                    items[i - 1].value + dp[i - 1][w - (items[i - 1].weight as usize)],
                );
                
                // retrieves twice: dp[i - 1][w] and dp[i - 1][w - (items[i - 1].weight as usize)]
                retrieved += 2;
            } else {
                dp[i][w] = dp[i - 1][w];
                retrieved += 1;
            }
        }
    }

    return (computed, retrieved);
}

fn dp_mem_count(items: Vec<Item>, capacity: usize) -> (i32, i32) {
    let n = items.len() as isize;
    let mut memo = vec![vec![-1; capacity + 1]; n as usize];
    let mut computed = 0;
    let mut retrieved = 0;
    let _ = dp_mem_rec_count(
        &items,
        capacity,
        n - 1,
        &mut memo,
        &mut computed,
        &mut retrieved,
    );

    return (computed, retrieved);
}

fn dp_mem_rec_count(
    items: &Vec<Item>,
    capacity: usize,
    index: isize,
    memo: &mut Vec<Vec<i32>>,
    computed: &mut i32,
    retrieved: &mut i32,
) -> i32 {
    if index < 0 {
        return 0;
    }

    if memo[index as usize][capacity] != -1 {
        *retrieved += 1;
        return memo[index as usize][capacity];
    }

    let result = if items[index as usize].weight as usize > capacity {
        dp_mem_rec_count(items, capacity, index - 1, memo, computed, retrieved)
    } else {
        std::cmp::max(
            dp_mem_rec_count(items, capacity, index - 1, memo, computed, retrieved),
            items[index as usize].value
            + dp_mem_rec_count(
                items,
                capacity - items[index as usize].weight as usize,
                index - 1,
                memo,
                computed,
                retrieved,
            ),
        )
    };
    
    *computed += 1;
    memo[index as usize][capacity] = result;

    return result;
}
