// https://leetcode.com/problems/sum-of-digits-in-base-k/
// 2 ms, 2 MB

impl Solution {
    pub fn sum_base(mut n: i32, k: i32) -> i32 {
        let mut converted = vec![];
        loop {
            let m = n % k;
            n = n / k;
            converted.push(std::char::from_digit(m as u32, k as u32).unwrap());
            if n == 0 {
                break;
            }
        }
        return addDigits(converted.into_iter().rev().collect::<String>().parse::<i32>().unwrap());
    }
}

pub fn addDigits(num: i32) -> i32 {
    if num >= 10 {
        return num % 10 + addDigits(num / 10);
    }
    return num;
}
