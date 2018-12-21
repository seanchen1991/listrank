use std::mem;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

const NULL: i64 = !0;

pub fn gen_list_and_head(n: usize) -> (i64, Vec<i64>) {
    let mut rng = thread_rng();
    let mut list: Vec<i64> = (0..n as i64).collect();

    list.shuffle(&mut rng);

    let rand_index: usize = rng.gen_range(0, n-1);

    let mut head = NULL;

    mem::swap(&mut list[rand_index], &mut head); 

    (head, list)
}

pub fn seq_listrank(head: i64, next: &[i64]) -> Vec<i64> {
    let mut head = head;
    let mut rank = 0;
    let mut ranks = vec![0; next.len()];

    while head != NULL {
        ranks[head as usize] = rank;
        rank += 1;
        head = next[head as usize];
    }

    ranks
}