use std::collections::HashSet;

pub fn euler_0003() -> String {
    let mut n: i64 = 600851475143;
    if n >= 1 << 52 {
        panic!("Number '{}' too big!", n);
    }

    let mut primes = HashSet::<i64>::new();
    if n == 1 {
        primes.insert(1);
        return format_primes(primes);
    }

    while n % 2 == 0 {
        primes.insert(2);
        n /= 2;
    }

    let sqrt_n_floor = (n as f64).sqrt().floor() as i64;
    (3..sqrt_n_floor)
        .into_iter()
        .step_by(2)
        .for_each(|i| {
            if n % i == 0 {
                primes.insert(i);
                n /= i;
            }
        });

    if n > 2 {
        primes.insert(n);
    }

    return format_primes(primes);
}

fn format_primes(primes: HashSet<i64>) -> String {
    let mut sorted_primes = primes.into_iter().collect::<Vec<_>>();
    sorted_primes.sort();

    return format!("{:?}", sorted_primes);
}
