pub fn solve() -> i64 {
    let mut sum_of_squares: i64 = 0;
    let mut square_of_sum: i64 = 0;
    for i in 1..=100 {
        sum_of_squares += i * i;
        square_of_sum += i;
    }
    return square_of_sum.pow(2) - sum_of_squares 
}
