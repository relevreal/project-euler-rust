const MAX_NUM: i64 = 4_000_000;

pub fn solve() -> i64 {
    let mut sum = 2;
    let mut num2 = 1;
    let mut num1 = 2;
    let mut tmp;
    
    loop {
        tmp = num1;
        num1 += num2;
        num2 = tmp;
        if num1 >= MAX_NUM {
            break;
        }
        if num1 % 2 == 1 {
            continue;
        }
        sum += num1;
        println!("{}", num1);
    }
    return sum;
}
