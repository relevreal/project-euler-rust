use crate::problem_3::is_prime;

// solution: 142913828922
pub fn solve() -> i64 {
    let mut sum: i64 = 2;
    for i in 3..=2_000_000 {
        if is_prime(i) {
            sum += i;
        } 
    }
    return sum
}
