// https://leetcode.com/problems/add-digits/
// 0 ms, 2 MB

impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        while num > 9 {
            num = digitSum(num);
        }
        return num;
    }
}

pub fn digitSum(mut num: i32) -> i32 {
    let mut sum = 0;
    sum += (num % 10);
	if num > 9 {
        sum += digitSum(num / 10);
    }
    return sum;
}
