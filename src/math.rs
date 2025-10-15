pub fn binomial_coefficient(numerator: u128, denominator: u128) -> u128 {
    let mut res: u128 = 1;
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

    for i in (1..=k).collect::<Vec<u128>>() {
        res *= n;
        res /= i;

        n -= 1;
    }

    res
}
