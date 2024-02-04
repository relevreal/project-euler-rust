const MAX_NUM: i64 = 100_000_000_000;
const STEP: i64 = 3 * 5 * 7 * 11 * 13 * 17 * 19;

pub fn solve() -> i64 {
    for i in (STEP..MAX_NUM).step_by(STEP as usize) {
        if is_divisible(i) {
            return i
        }
    }
    return -1
}

fn is_divisible(num: i64) -> bool {
    for i in 1..=20 {
        if num % i != 0 {
            return false
        }
    }
    return true
}
