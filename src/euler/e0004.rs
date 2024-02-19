use std::collections::HashSet;

pub fn euler_0004() -> String {
    let digits = 3;
    let max_denom = get_max_denom(digits);
    let min_denom = get_max_denom(digits - 1) + 1;

    let mut palindromes = HashSet::<String>::new();
    let mut s: String = Default::default();
    (min_denom..max_denom).rev().for_each(|i| {
        (min_denom..max_denom).rev().for_each(|j| {
            s = (i * j).to_string();
            if is_a_palindrome(&s) {
                palindromes.insert(s.clone());
            }
        });
    });

    let mut sorted_palindromes = palindromes.iter().collect::<Vec<_>>();
    sorted_palindromes.sort();

    format!("{:?}", sorted_palindromes.last())
}

fn get_max_denom(digits: i32) -> i32 {
    let max_str: String = (0..digits)
        .into_iter()
        .map(|_| "9")
        .collect::<String>();

    match max_str.as_str().parse::<i32>() {
        Ok(max) => max,
        Err(error) => panic!("Error initializing max denominator: {}", error),
    }
}

fn is_a_palindrome(s: &String) -> bool {
    let len = s.len();
    let left: String = s[0..len / 2].chars().collect();
    let right: String = s[len / 2..len].chars().rev().collect();

    left == right
}

