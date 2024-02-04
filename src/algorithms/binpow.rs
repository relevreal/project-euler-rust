pub fn binpow_recursive(mut a: i64, mut b: i64) -> i64 {
    if b == 0 {
        return 1
    }
    let res = binpow_recursive(a, b / 2);
    if b % 2 == 1 {
        return res * res * a;
    } 
    return res * res
}

// Non tail-recursive
pub fn exp_by_squaring(x: f64, n: i64) -> f64 {
    assert!(x != 0.0);
    if n < 0 {
        return exp_by_squaring(1.0 / x, -n)
    }
    if n == 0 {
        return 1.0
    }
    if n % 2 == 0 {
        return exp_by_squaring(x * x, n / 2)
    }
    return x * exp_by_squaring(x * x, (n - 1) / 2)
}

// tail-recursive
pub fn exp_by_squaring2(x: f64, n: i64) -> f64 {
    return exp_by_squaring22(1.0, x, n);
}

pub fn exp_by_squaring22(y: f64, x: f64, n: i64) -> f64 {
    if n < 0 {
        return exp_by_squaring22(y, 1.0 / x, -n)
    }
    if n == 0 {
        return y
    }
    if n % 2 == 0 {
        return exp_by_squaring22(y, x * x, n / 2)
    }
    return exp_by_squaring22(x * y, x * x, (n - 1) / 2)
}

pub fn exp_by_squaring_iterative(mut x: f64, mut n: i64) -> f64 {
    if n < 0 {
        x = 1.0 / x;
        n = -n; 
    }
    if n == 0 {
        return 1.0
    }
    let mut y = 1.0;
    loop {
        if n <= 1 {
            break
        }
        if n % 2 == 0 {
            x = x * x;
            n = n / 2;
        } else {
            y = x * y;
            x = x * x;
            n = (n - 1) / 2;
        }
    }
    return x * y
}

pub fn binpow(mut a: i64, mut b: i64) -> i64 {
    let mut res: i64 = 1;
    loop {
        if b <= 0 {
            break;
        }
        if b & 1 == 1 {
            res *= a;
        }
        a *= a;
        b >>= 1;
    }
    return res
}

// Sample Input
// 3
// 18132
// 17
// 17
// 1765
// 3
// 2374859
// 3029382
// 36123
// Sample Output
// 13
// 2
// 13195
pub fn binpow_mod(mut a: i64, mut b: i64, m: i64) -> i64 {
    a %= m;
    let mut res: i64 = 1;
    loop {
        if b <= 0 {
            break
        }
        if b & 1 == 1 {
            res = res * a % m;
        }
        a = a * a % m;
        b >>= 1;
    }
    return res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binpow_recursive() {
        let expected = 3_i64.pow(11);
        let result = binpow_recursive(3, 11);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_binpow() {
        let expected = 3_i64.pow(11);
        let result = binpow(3, 11);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_binpow_mod() {
        let expected1 = 3;
        let expected2 = 11;
        let result1 = binpow_mod(2, 3, 5);
        let result2 = binpow_mod(2, 2147483647, 13);
        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);

        println!("{}, {}", binpow_mod(123456, 1, 1000), binpow_mod(123456, 2, 1000));
    }

    #[test]
    fn test_exp_by_squaring() {
        let expected = 3_f64.powi(11);
        let result = exp_by_squaring(3.0, 11);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_exp_by_squaring2() {
        let expected = 3_f64.powi(11);
        let result = exp_by_squaring2(3.0, 11);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_exp_by_squaring_iterative() {
        let expected = 3_f64.powi(11);
        let result = exp_by_squaring_iterative(3.0, 11);
        assert_eq!(result, expected);
    }
}
