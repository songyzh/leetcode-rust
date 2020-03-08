use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub fn get_sorted_vec() -> Vec<i32> {
    (0..10).collect()
}

pub fn get_shuffled_vec() -> Vec<i32> {
    let mut v = get_sorted_vec();
    v.shuffle(&mut thread_rng());
    v
}
