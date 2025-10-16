use itertools::Itertools;

pub fn binomial_coefficient(numerator: usize, denominator: usize) -> usize {
    let mut res: usize = 1;
    let mut n = numerator;
    let mut k = denominator;

    if n < k || n == 0 {
        return 0;
    }

    if n == k || k == 0 {
        return res;
    }

    if n < k * 2 {
        k = n - k;
    }

    for i in (1..=k).collect::<Vec<usize>>() {
        res *= n;
        res /= i;

        n -= 1;
    }

    res
}

pub fn combinations<T: Default + Copy>(
    set: &Vec<T>,
    set_fixed: &Vec<T>,
    n_seq: usize,
) -> Result<Vec<Vec<T>>, ()> {
    let n_set = set.len();
    let n_fixed = set_fixed.len();

    if n_set == 0 || n_seq <= n_fixed {
        return Err(());
    }

    let n_rand = n_seq - n_fixed;
    let n_combs = binomial_coefficient(n_set, n_rand);
    let mut combs: Vec<Vec<T>> = Vec::new();
    combs.reserve_exact(n_combs);

    for set_comb in set.into_iter().combinations(n_rand) {
        let mut comb: Vec<T> = Vec::new();
        comb.reserve_exact(n_seq);

        for v in set_fixed {
            comb.push(*v);
        }

        for v in set_comb {
            comb.push(*v);
        }

        combs.push(comb);
    }

    Ok(combs)
}
