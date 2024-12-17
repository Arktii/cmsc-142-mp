pub mod item;
pub mod items;
pub mod recorder;

use item::Item;
use items::ITEM_SETS;
use recorder::{IterationResult, Record, Recorder};
use std::error::Error;
use std::time::Instant;

const START_N: usize = 100;
const MAX_N: usize = 10000;
const STEP: usize = 100;
const KNAPSACK_CAPACITY: usize = 1000;

// struct Record {
//     value: i32,
//     time: u128,
// }

fn main() -> Result<(), Box<dyn Error>> {
    // let mut wtr = Writer::from_path("results/results.csv")?;

    // wtr.write_record(&[
    //     "n",
    //     "value 1",
    //     "time 1",
    //     "value 2",
    //     "time 2",
    //     "value 3",
    //     "time 3",
    //     "average time",
    // ])?;

    let mut recorder = Recorder::new("results/results.csv");
    recorder.default_setup()?;

    for n in (START_N..=MAX_N).step_by(STEP) {
        let mut iteration_result: IterationResult = IterationResult::new(n);

        for i in 0..3 {
            let items = ITEM_SETS[i][0..n].to_vec();

            // TODO: handle stack overflow error
            // DP Memoization
            run_trial(&items, &mut iteration_result.dp_mem, |items, capacity| {
                dp_mem_solve(items, capacity)
            });

            // DP Tabulation
            run_trial(&items, &mut iteration_result.dp_tab, |items, capacity| {
                dp_tab_solve(items, capacity)
            });

            // Greedy Highest Value
            run_trial(
                &items,
                &mut iteration_result.greedy_highest_value,
                |items, capacity| {
                    let compare = |a: &Item, b: &Item| b.value.cmp(&a.value);
                    greedy_solve(items, capacity, compare)
                },
            );

            // Greedy Smallest Weight
            run_trial(
                &items,
                &mut iteration_result.greedy_smallest_weight,
                |items, capacity| {
                    let compare = |a: &Item, b: &Item| a.weight.cmp(&b.weight);
                    greedy_solve(items, capacity, compare)
                },
            );

            // Greedy Greatest Ratio
            run_trial(
                &items,
                &mut iteration_result.greedy_greatest_ratio,
                |items, capacity| {
                    let compare = |a: &Item, b: &Item| (b.value / b.weight).cmp(&(a.value / a.weight));
                    greedy_solve(items, capacity, compare)
                },
            );
        }

        recorder.write_iteration_result(iteration_result)?;

        println!("Finished iteration {}", n);
    }
    Ok(())
}

fn run_trial(
    items: &Vec<Item>,
    results: &mut Vec<Record>,
    algorithm: fn(&mut Vec<Item>, usize) -> i32,
) {
    let mut items_copy = items.clone();

    let start = Instant::now();
    let solution_value = algorithm(&mut items_copy, KNAPSACK_CAPACITY);
    let duration = start.elapsed();

    results.push(Record::new(solution_value as usize, duration.as_micros()));
}

// fn write_to_csv(wtr: &mut Writer<File>, n: usize, iteration_data: &[Record]) {
//     let values: Vec<String> = iteration_data
//         .iter()
//         .flat_map(|record| vec![record.value.to_string(), record.time.to_string()])
//         .collect();

//     let sum_time: u128 = iteration_data.iter().map(|r| r.time).sum();
//     let avg_time: f64 = sum_time as f64 / iteration_data.len() as f64;

//     let mut record = vec![n.to_string()];
//     record.extend(values);
//     record.push(avg_time.to_string());

//     wtr.write_record(&record).unwrap();
//     wtr.flush().unwrap();
// }

// https://www.geeksforgeeks.org/0-1-knapsack-problem-dp-10/#tabulation-or-bottomup-approach-on-x-w-time-and-space
fn dp_tab_solve(items: &Vec<Item>, capacity: usize) -> i32 {
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
fn dp_mem_solve(items: &Vec<Item>, capacity: usize) -> i32 {
    let n = items.len() as isize;
    let mut memo = vec![vec![-1; capacity + 1]; n as usize];

    return dp_mem_rec(items, capacity, n - 1, &mut memo);
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
    items: &mut Vec<Item>,
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
