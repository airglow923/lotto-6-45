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
    set: &[T],
    set_fixed: Option<&[T]>,
    n_seq: usize,
) -> Result<Vec<Vec<T>>, String> {
    let n_set = set.len();
    let n_fixed = match set_fixed {
        Some(t) => t.len(),
        None => 0,
    };

    if n_set == 0 || n_seq <= n_fixed {
        return Err(
            "set cannot be empty and n_seq should be greater than length of set_fixed".to_owned(),
        );
    }

    let n_rand = n_seq - n_fixed;
    let n_combs = binomial_coefficient(n_set, n_rand);
    let mut combs: Vec<Vec<T>> = Vec::new();
    combs.reserve_exact(n_combs);

    for set_comb in set.into_iter().combinations(n_rand) {
        let mut comb: Vec<T> = Vec::new();
        comb.reserve_exact(n_seq);

        if n_fixed != 0 {
            for v in set_fixed.unwrap() {
                comb.push(*v);
            }
        }

        for v in set_comb {
            comb.push(*v);
        }

        combs.push(comb);
    }

    Ok(combs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial_coefficient_tn_zero_numerator() {
        let ret = binomial_coefficient(0, 1);

        assert_eq!(ret, 0);
    }

    #[test]
    fn test_binomial_coefficient_tp_zero_denominator() {
        let ret = binomial_coefficient(1, 0);

        assert_eq!(ret, 1);
    }

    #[test]
    fn test_binomial_coefficient_tp_equal_n_k() {
        let ret = binomial_coefficient(1, 1);

        assert_eq!(ret, 1);
    }

    #[test]
    fn test_binomial_coefficient_tn_greater_k() {
        let ret = binomial_coefficient(1, 2);

        assert_eq!(ret, 0);
    }

    #[test]
    fn test_binomial_coefficient_tp_valid() {
        let ret = binomial_coefficient(45, 6);

        assert_eq!(ret, 8145060);
    }

    #[test]
    fn test_combinations_tp_valid_slice() {
        let ret = combinations(&[1, 2, 3, 4, 5], Some(&[6, 7]), 3);
        assert!(ret.is_ok());

        let combs = ret.unwrap();
        assert_eq!(
            combs,
            vec![
                vec![6, 7, 1],
                vec![6, 7, 2],
                vec![6, 7, 3],
                vec![6, 7, 4],
                vec![6, 7, 5],
            ],
        );
        assert_eq!(combs.len(), binomial_coefficient(5, 1));
        assert_eq!(combs.capacity(), binomial_coefficient(5, 1));
        assert_eq!(combs[0].len(), 3);
        assert_eq!(combs[0].capacity(), 3);
    }

    #[test]
    fn test_combinations_tp_valid_vec() {
        let ret = combinations(&vec![1, 2, 3, 4, 5], Some(&vec![6, 7]), 3);
        assert!(ret.is_ok());

        let combs = ret.unwrap();
        assert_eq!(
            combs,
            vec![
                vec![6, 7, 1],
                vec![6, 7, 2],
                vec![6, 7, 3],
                vec![6, 7, 4],
                vec![6, 7, 5],
            ],
        );
        assert_eq!(combs.len(), binomial_coefficient(5, 1));
        assert_eq!(combs.capacity(), binomial_coefficient(5, 1));
        assert_eq!(combs[0].len(), 3);
        assert_eq!(combs[0].capacity(), 3);
    }

    #[test]
    fn test_combinations_tp_no_fixed() {
        let ret = combinations(&[1, 2, 3, 4, 5], None, 3);
        assert!(ret.is_ok());

        let combs = ret.unwrap();
        assert_eq!(
            combs,
            vec![
                vec![1, 2, 3],
                vec![1, 2, 4],
                vec![1, 2, 5],
                vec![1, 3, 4],
                vec![1, 3, 5],
                vec![1, 4, 5],
                vec![2, 3, 4],
                vec![2, 3, 5],
                vec![2, 4, 5],
                vec![3, 4, 5],
            ],
        );
        assert_eq!(combs.len(), binomial_coefficient(5, 3));
        assert_eq!(combs.capacity(), binomial_coefficient(5, 3));
        assert_eq!(combs[0].len(), 3);
        assert_eq!(combs[0].capacity(), 3);
    }

    #[test]
    fn test_combinations_tp_45_6() {
        let ret = combinations(&(1..=45).collect::<Vec<_>>(), None, 6);
        assert!(ret.is_ok());

        let combs = ret.unwrap();
        assert_eq!(combs.len(), binomial_coefficient(45, 6));
        assert_eq!(combs.capacity(), binomial_coefficient(45, 6));
        assert_eq!(combs[0].len(), 6);
        assert_eq!(combs[0].capacity(), 6);

        println!("{:?}", &combs[..10]);
    }
}
