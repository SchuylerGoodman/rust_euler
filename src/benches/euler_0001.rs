fn main() {
    let max = 1000;

    let sum: i32 = (0..max)
        .into_iter()
        .filter(|i| {
            i % 3 == 0 || i % 5 == 0
        })
        .sum();

    println!("{}", sum);
}
