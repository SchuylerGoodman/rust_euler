pub fn euler_0009() -> String {
    let t = 1000;
    let max = t / 2;
    let mut result: u32 = 0;
    (1..max + 1).for_each(|a: u32| {
        (1..max + 1).for_each(|b: u32| {
            let a_f = f64::from(a);
            let b_f = f64::from(b);
            let x = (500000.0 - 1000.0 * b_f) / a_f + b_f;
            if x == 1000.0 {
                let c: u32 = 1000 - a - b;
                result = a * b * c;
            }
        })
    });

    if result > 0 {
        result.to_string()
    }
    else {
        "Could not find result".to_owned()
    }
}