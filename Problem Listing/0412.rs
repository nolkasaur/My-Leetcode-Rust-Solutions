// https://leetcode.com/problems/fizz-buzz/
// 5 ms, 2.7 MB

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res:Vec<String> = Vec::with_capacity(n as usize);
        for x in 1..n+1 {
            if x%3==0 {
                if x%5==0 {
                    res.push(String::from("FizzBuzz"));
                } else {
                    res.push(String::from("Fizz"));
                }
            } else if x%5==0 {
                res.push(String::from("Buzz"));
            } else {
                res.push(x.to_string());
            }
        }
        return res;
    }
}
