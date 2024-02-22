use num::BigInt;

pub fn euler_0015() -> String {
    let n: u128 = 20;
    let number_of_paths = factorial(&(n * 2).into()) / factorial(&n.into()).pow(2);
    number_of_paths.to_string()
}

fn factorial(num: &BigInt) -> BigInt {
    if *num <= 1.into() {
        return 1.into();
    }

    let sub = num - BigInt::from(1);
    factorial(&sub) * num
}

/*
pub fn euler_0015_slow(n: i32) -> String {
    mv(0, 0, n).to_string()
}

fn mv(downs: i32, rights: i32, n: i32) -> u128 {
    if downs >= n || rights >= n {
        return u128::from_i32(1).expect("Why would this ever fail?");
    }

    mv(downs + 1, rights, n) + mv(downs, rights + 1, n)
}
*/