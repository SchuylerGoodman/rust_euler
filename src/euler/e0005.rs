pub fn euler_0005() -> String {
    let min = 1;
    let max = 20;
    assert!(min > 0);
    assert!(max > 1);

    let too_big = 999999999;
    let mut current = max;
    let mut div: i32 = max - 1;
    while current < too_big {
        while div >= min && (current % div) == 0 {
            div -= 1;
        }

        if div < min {
            return current.to_string();
        }

        div = max - 1;
        current += max;
    }

    format!("Could not find a number easily divisible by {} through {} lower than {}", min, max, too_big)
}