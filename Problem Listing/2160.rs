// https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/
// 1ms, 2.1 MB

impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut minA = num as u32;
        let mut minB = num as u32;
        let mut maxA = 0;
        let mut maxB = 0;
        for digit in num.to_string().chars() {
            let d = digit.to_digit(10).unwrap();
            if minA >= minB {
                if d < minA {
                    if minA != (num as u32) {
                        if maxA == 0 {
                            maxA = minA;
                        } else {
                            maxB = minA;
                        }
                    }
                    minA = d;
                } else {
                    if maxA == 0 {
                        maxA = d;
                    } else {
                        maxB = d;
                    }
                }
            } else {
                if d < minB {
                    if minB != (num as u32) {
                        if maxB == 0 {
                            maxB = minB;
                        } else {
                            maxA = minB;
                        }
                    }
                    minB = d;
                } else {
                    if maxB == 0 {
                        maxB = d;
                    } else {
                        maxA = d;
                    }
                }
            }
        }
        let mut numA = String::from("");
        numA.push(char::from_digit(minA, 10).unwrap());
        numA.push(char::from_digit(maxA, 10).unwrap());
        let mut numB = String::from("");
        numB.push(char::from_digit(minB, 10).unwrap());
        numB.push(char::from_digit(maxB, 10).unwrap());
        return numA.parse::<i32>().unwrap() + numB.parse::<i32>().unwrap();
    }
}
