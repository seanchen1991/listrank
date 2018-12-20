use std::mem;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

const NULL: i64 = !0;

fn gen_list_and_head(n: usize) -> (i64, Vec<i64>) {
    let mut rng = thread_rng();
    let mut pi: Vec<i64> = (0..n as i64).collect();

    pi.shuffle(&mut rng);

    let mut next: Vec<i64> = vec![0; n]; 

    for i in 0..n-1 {
        mem::replace(&mut next[pi[i] as usize], pi[i+1].clone());
    }

    mem::replace(&mut next[pi[n-1] as usize], NULL);

    (pi[0].clone, next)
}

fn seq_listrank(list: &[i64]) -> Vec<i64> {
    let mut rank = 0;
    let mut countdown = list.len();
    let mut ranks = Vec::with_capacity(list.len());

    while countdown >= 0 {
        ranks.push()
    }
}

pub fn seq_list_rank<T: PartialEq>(head: usize, next: &[Option<T>]) -> Vec<usize> {
    let mut r = 0;
    let mut ranks: Vec<usize> = vec![0; n];

    loop {
        match head as_ref {
            Some(_) => {
                r += 1;
                ranks[head.unwrap()] = r;
                head = &mut next[head.unwrap()];
            }
            None => { break; }
        }
    }

    ranks
}

#[cfg(test)]
mod tests {
    use gen_list;

    #[test]
    fn test_gen_list_length() {
        let length = 50;
        let list = gen_list(50);
        assert_eq!(list.len(), length);
    }
}