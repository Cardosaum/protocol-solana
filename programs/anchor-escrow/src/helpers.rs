use super::*;
use itertools::Itertools;
use solana_program::log::sol_log_compute_units;

fn ackermann(m: usize, n: usize) -> usize {
    if m == 0 {
        return n + 1;
    } else if n == 0 {
        return ackermann(m - 1, 1);
    } else {
        return ackermann(m - 1, ackermann(m, n - 1));
    }
}

pub fn call_ackermann() {
    for (i, j) in vec![0, 0, 1, 1, 1, 2, 2, 1, 2, 2, 3, 4]
        .into_iter()
        .tuples()
    {
        msg!("> Ackerman({}, {})", i, j);
        ackermann(i, j);
        sol_log_compute_units();
    }
}

pub fn consume_all_compute_units() {
    let mut bruh = 0_usize;
    let mut what = 3_usize;
    let mut vish = vec![40_usize, 1_000_000];
    loop {
        sol_log_compute_units();
        if what != 0 {
            if let Some(x) = 23_usize.checked_add(what) {
                if let Some(y) = bruh.checked_add(x) {
                    bruh = y;
                }
            }
        }
        what *= bruh + 19 + what;
        let argh = vish.clone();
        let eita = vish.clone();
        let termina = vish.clone();
        let ai = vish.clone();
        for i in vish.iter_mut() {
            if *i <= 1_000_000_000 {
                if let Some(x) = i.checked_mul(what) {
                    *i = x;
                }
            }
            for j in argh.iter() {
                for g in eita.iter() {
                    for h in termina.iter() {
                        for u in ai.iter() {
                            if let Some(x) = i.checked_mul(*j) {
                                if let Some(y) = x.checked_mul(*g) {
                                    if let Some(z) = y.checked_sub(*h) {
                                        if let Some(w) = z.checked_rem(*u) {
                                            *i = w;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn stack_bomb() -> Vec<[usize; 1_000]> {
    return vec![[0_usize; 1_000]; 1_000];
}
