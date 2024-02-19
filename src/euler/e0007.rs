pub fn euler_0007() -> String {
    let target = 10001;
    let max = 9999999;

    let mut is_prime = vec![true; max];
    (1..max + 1).for_each(|i| {
        if i != 1 && is_prime[i - 1] {
            (2*i..max + 1)
                .into_iter()
                .step_by(i)
                .for_each(|j| is_prime[j - 1] = false);
        }
    });

    let primes = is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, p)| p.then(|| i + 1))
        .collect::<Vec<_>>();

    match primes.get(target) {
        Some(i) => (i).to_string(),
        None => "Max is not large enough".to_owned(),
    }
}