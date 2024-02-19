// The naive solution, works well enough
pub fn euler_0006() -> String {
    let min = 1;
    let max = 100 + 1;

    let square_of_sums = (min..max).sum::<i32>().pow(2);
    let sum_of_squares = (min..max)
        .map(|i: i32| i.pow(2))
        .sum::<i32>();

    (square_of_sums - sum_of_squares).to_string()
}
