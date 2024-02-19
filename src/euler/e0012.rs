pub fn euler_0012() -> String {
    let target_divisors = 500;
    let max = 12377;

    let mut current_triangle = 0;
    let mut divisor_count = 0;
    for i in 1..max {
        current_triangle = (i * (i + 1)) / 2;
        if i % 2 == 0 {
            divisor_count = count_divisors(i / 2, 1) * count_divisors(i + 1, 1);
        }
        else {
            divisor_count = count_divisors(i, 1) * count_divisors((i + 1) / 2, 1);
        }
        println!("{} {} {}", i, current_triangle, divisor_count);

        if divisor_count > target_divisors {
            break;
        }
    }

    match (divisor_count > target_divisors).then_some(current_triangle) {
        Some(result) => result.to_string(),
        None => "Could not find the result".to_owned(),
    }
}

fn count_divisors(n: i32, start: i32) -> usize {
    let max = f64::from(n).sqrt().ceil() as i32;
    if n == 1 {
        return 1;
    }

    let mut n_div = n;
    for i in (start..max + 1) {
        if n_div % i == 0 {
            let mut count = 1;
            while n_div % i == 0 {
                n_div /= i;
                count += 1;
            }

            return count_divisors(n_div, i + 1) * count;
        }
    }

    2
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
