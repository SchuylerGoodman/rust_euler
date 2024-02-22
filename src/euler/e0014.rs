use std::collections::HashMap;

pub fn euler_0014() -> String {
    let mut start: usize = 1000000;
    let mut length_cache = HashMap::<usize, usize>::new();
    let mut max_length = 0;
    let mut max_start = start;
    while start > 1 {
        let length = get_length(start, &mut length_cache);
        if length > max_length {
            max_length = length;
            max_start = start;
        }

        start -= 1;
    }

    max_start.to_string()
}

fn get_length(current: usize, length_cache: &mut HashMap<usize, usize>) -> usize {
    if current < 1 {
        panic!("Trying to get chain length from '{}', should not be possible.", current);
    }

    if current == 1 {
        return 1;
    }

    if !length_cache.contains_key(&current) {
        let new_current = collatz(current);
        let new_current_length = get_length(new_current, length_cache) + 1;
        length_cache.insert(current, new_current_length);
    }

    match length_cache.get(&current) {
        Some(length) => *length,
        None => panic!("No entry for key '{}' in length cache. This shouldn't happen.", current)
    }
}

fn collatz(n: usize) -> usize {
    match n {
        n if n % 2 == 0 => n / 2,
        n => 3 * n + 1,
    }
}
