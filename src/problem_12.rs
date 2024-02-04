const MAX_NUM: i64 = 2_000_000;
const FAC_LEN: usize = 500;

pub fn solve() -> i64 {
    let mut triangle_num: i64 = 0;
    for i in 1..=MAX_NUM {
        triangle_num += i;
        let factors_len = get_factor_num(triangle_num) as usize;
        println!("{}: {}", triangle_num, factors_len);
        if factors_len == FAC_LEN {
            return triangle_num
        }
    }
    unreachable!()
}

fn get_factor_num(num: i64) -> usize {
    // let mut factors: Vec<i64> = Vec::new();
    let mut sum: usize = 0;
    let num_sqrt = (num as f64).sqrt() as i64;
    for i in 1..=num_sqrt {
        if num % i == 0 {
            // factors.push(i);
            sum += 1;
        }
    }
    return sum
}
