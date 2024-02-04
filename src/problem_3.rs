const NUMBER: i64 = 600851475143;

pub fn solve() -> i64 {
    let upper_boundary = (NUMBER as f64).sqrt() as i64;
    for i in (2..=upper_boundary).rev() {
        if NUMBER % i == 0 && is_prime(i){
            return i
        }
    }
    return 1
}

pub fn is_prime(num: i64) -> bool {
    let upper_bound = (num as f64).sqrt() as i64;
    for i in 2..=upper_bound {
        if num % i == 0 {
            return false
        }
    }
    return true
}
