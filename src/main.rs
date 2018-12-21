extern crate listrank;

use listrank::*;
use rayon::prelude::*;

fn update_ranks(ranks: Vec<i64>, next: &[i64]) -> Vec<i64> {
    let mut r2: Vec<i64> = (0..ranks.len()).into_par_iter().collect();
    let par_iter = (0..ranks.len()).into_par_iter();

    for i in par_iter {
        if ranks[i] != NULL {
            r2[ranks[i]] = ranks[i] + ranks[next[i]];
        }
    }

    ranks.into_par_iter()
        .map(|i| {
            r2[i]
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

    let mut r2 = r1.clone();

    let mut n1: Vec<i64> = (0..n as usize)
        .into_par_iter()
        .map(|i| {
            next[i].clone()
        }).collect();

    let mut n2 = n1.clone();

    r1[head] = 0;
    r2[head] = 0;

    for _ in x {

    }
}