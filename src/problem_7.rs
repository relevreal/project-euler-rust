use crate::problem_3::is_prime;

const MAX_NUM: i64 = 1_000_000;
const PRIME_NUM: i64 = 10_001;

pub fn solve() -> i64 {
    let mut prime_count: i64 = 0;
    for i in (1..=MAX_NUM).step_by(2) {
        if is_prime(i) {
            prime_count += 1;    
            if prime_count == PRIME_NUM {
                return i
            }
        } 
    }
    return -1;
}
