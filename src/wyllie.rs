extern crate listrank;

use listrank::*;
use rayon::prelude::*;

fn jump_list(next: Vec<i64>) -> Vec<i64> {
    let mut n2: Vec<i64> = vec![0; next.len()];
    let par_iter = (0..next.len()).into_par_iter();

    for i in par_iter {
        if next[i] != NULL {
            n2[i] = next[next[i] as usize];
        }
    }

    let next = next.into_par_iter()
        .map(|i| {
            n2[i as usize]
        }).collect();

    next
}

fn update_ranks(ranks: Vec<i64>, next: &[i64]) -> Vec<i64> {
    let mut r2: Vec<i64> = vec![0; ranks.len()];
    let par_iter = (0..ranks.len()).into_par_iter();

    for i in par_iter {
        if ranks[i] != NULL {
            r2[ranks[i]] = ranks[i] + ranks[next[i]];
        }
    }

    let ranks = ranks.into_par_iter()
        .map(|i| {
            r2[i as usize]
        }).collect();

    ranks
}

fn wyllie_listrank(head: usize, next: &[i64]) -> Vec<i64> {
    let n = next.len() as f64;
    let num_jumps = (n.ln() / 2_f64.ln()).ceil() as usize;

    let mut r1: Vec<i64> = (0..n as usize)
        .into_par_iter()
        .map(|_| {
            1
        }).collect();

    let mut n1: Vec<i64> = (0..n as usize)
        .into_par_iter()
        .map(|i| {
            next[i].clone()
        }).collect();

    r1[head] = 0;

    for _ in x {
        r1 = update_ranks(r1, &n1);
        n1 = copy_over(n1);
    }

    r1
}