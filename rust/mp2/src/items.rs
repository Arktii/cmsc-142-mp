use crate::item::Item;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const ITEM_COUNT: usize = 100000;
const ITEM_SETS: usize = 3;

const MIN_WEIGHT: usize = 100;
const MAX_WEIGHT: usize = 1500;

const MIN_VALUE: usize = 100;
const MAX_VALUE: usize = 501;

const SEED: u64 = 20211422;

pub fn generate_items() -> Vec<Vec<Item>> {
    let mut rng = StdRng::seed_from_u64(SEED);

    let mut item_sets: Vec<Vec<Item>> = vec![];

    for _ in 0..ITEM_SETS {
        let mut item_set: Vec<Item> = vec![];
        for __ in 0..ITEM_COUNT {
            let weight = rng.gen_range(MIN_WEIGHT..MAX_WEIGHT) as i32;
            let value = rng.gen_range(MIN_VALUE..MAX_VALUE) as i32;
            item_set.push(Item { weight, value });
        }
        item_sets.push(item_set);
    }

    item_sets
}
