pub fn solve() -> i64 {
    for a in 1..=333 as i64 {
        for b in a..=998 as i64 {
            let c = 1000 - a - b;
            // println!("{a} + {b} + {c} = {}", a + b + c);
            // println!("{}, {}, {}", a.pow(2), b.pow(2), c.pow(2));
            if c > b && a.pow(2) + b.pow(2) == c.pow(2) {
                println!("{a} {b} {c}");
                return a * b * c
            }
        }
    }
    return -1 
}
