use std::iter::zip;

pub fn solve() -> i64 {
    let mut largest = 10001;
    for i in 100..=999 {
        for ii in 100..=999 {
            let num_str = (i * ii).to_string();
            if is_palindrome(&num_str) {
                println!("{} * {} = {}", i, ii, i * ii);
                if i * ii > largest {
                    largest = i * ii;
                }
            }
        }
    }
    return largest
}

fn is_palindrome(num_str: &str) -> bool {
    let len = num_str.len();
    let x: usize;
    let y: usize;
    if len % 2 == 0 {
        x = len / 2;
        y = x;
    } else {
        x = len / 2;
        y = x + 1;
    }
    let first = num_str[..x].chars();
    let reversed = num_str[y..].chars().rev();
    
    for (i, ii) in zip(first, reversed) {
        if i != ii {
            return false
        }
    }
    return true
}
