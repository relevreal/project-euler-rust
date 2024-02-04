pub fn solve() -> i64 {
    let mut sum = 0;
    for i in 0..1000 {
        if i % 5 == 0 || i % 3 == 0 {
            sum += i;
        }
    }
    return sum
}
