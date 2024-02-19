pub fn euler_0012() -> String {
    let target_divisors = 500;
    let max = 13000;

    let mut current_triangle = 0;
    let mut divisor_count = 0;
    for n in 1..max {
        current_triangle = (n * (n + 1)) / 2;
        if n % 2 == 0 {
            divisor_count = count_divisors(n / 2) * count_divisors(n + 1);
        } else { 
            divisor_count = count_divisors(n) * count_divisors((n + 1) / 2);
        }

        if divisor_count > target_divisors {
            break;
        }
    }

    match (divisor_count > target_divisors).then_some(current_triangle) {
        Some(result) => result.to_string(),
        None => "Could not find the result".to_owned(),
    }
}

fn count_divisors(n: i32) -> usize {
    let max = f64::from(n).sqrt().ceil() as i32;
    if n == 1 {
        return 1;
    }

    let mut n_div = n;
    let mut total = 1;
    for i in 2..max + 1 {
        let mut count = 1;
        while n_div % i == 0 {
            n_div /= i;
            count += 1;
        }

        total *= count;
    }

    total
}

/*
#[allow(dead_code)]
pub fn euler_0012_brute_force() -> String {
    let mut triangles: Vec<i32> = vec!();
    let target_divisors = 500;

    let max = i32::MAX;
    let mut current_triangle = 0;
    for i in 1..max {
        current_triangle += i;
        triangles.push(current_triangle);

        if count_divisors(current_triangle) > target_divisors {
            break;
        }
    }

    current_triangle.to_string()
}
*/
