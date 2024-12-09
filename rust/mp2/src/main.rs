mod items;

use std::time::Instant;

use items::{Item, ITEM_SETS};

const START_N: usize = 10;
const MAX_N: usize = 50;
const KNAPSACK_CAPACITY: usize = 1000;

fn main() {
    println!("Hello, world!");

    for n in START_N..=MAX_N {
        for i in 0..3 {
            let items = ITEM_SETS[i].to_vec();

            let start = Instant::now();

            // let (solution, solution_weight, solution_value) = brgc_knapsack(&items, n);
            // let solution_value = dp_tab_solve(&items[0..n].to_vec(), KNAPSACK_CAPACITY);
            let solution_value = dp_mem_solve(&items[0..n].to_vec(), KNAPSACK_CAPACITY);
            let duration = start.elapsed();
            println!(
                "n: {}, i: {}, value: {}, time: {}.{:09} seconds",
                n,
                i + 1,
                solution_value,
                duration.as_secs(),
                duration.subsec_nanos()
            );
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

// todo: include way to pass in different evaluation/heuristic functions
fn greedy_solve(items: &Vec<Item>, n: usize, h: fn(&Item) -> f32) {}
