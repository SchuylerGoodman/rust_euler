pub fn euler_0010() -> String {
    let max = 2000000;
    let mut is_prime = vec![true; max];
    is_prime[0] = false;

    (1..max + 1).for_each(|i| {
        if i != 1 && is_prime[i - 1] {
            (2*i..max + 1)
                .into_iter()
                .step_by(i)
                .for_each(|j| is_prime[j - 1] = false);
        }
    });

    let sum_primes = is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, p)| p.then(|| i + 1))
        .sum::<usize>();

    sum_primes.to_string()
}