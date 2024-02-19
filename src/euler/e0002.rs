pub fn euler_0002_fast() -> String {
    let sqrt5: f64 = (5.0_f64).sqrt();
    let g_ratio: f64 = (1.0 + sqrt5) / 2.0;
    let max_x = 4000000.0;
    let max_i: i32 = (sqrt5 * max_x).log(g_ratio).floor() as i32;

    let sum: i32 = (0..max_i + 1)
        .into_iter()
        .step_by(3)
        .map(|i| (g_ratio.powi(i) / sqrt5).round() as i32)
        .sum();

    return sum.to_string();
}

#[allow(dead_code)]
pub fn euler_0002_slow() -> String {
    let max_x = 4000000;
    let mut x_1 = 0;
    let mut x_2 = 1;
    let mut x_swap: i32;
    let mut sum = 0;

    while x_2 < max_x {
        x_swap = x_2;
        x_2 = x_1 + x_2;
        x_1 = x_swap;

        if x_2 % 2 == 0 {
            sum += x_2;
        }
    }

    return sum.to_string();
}
