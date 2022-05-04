// https://leetcode.com/problems/count-largest-group/
// 6 ms, 2.1 MB

use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut sums: HashMap<i32, i32> = HashMap::new();
        for x in 1..n+1 {
            let c = sums.entry(sum_of_digits(x)).or_insert(0);
            *c+=1;
        }
        let mut largestSize = 0;
        let mut largestCount = 0;
        for (k,v) in sums.iter() {
            if *v > largestSize {
                largestSize = *v;
                largestCount = 1;
            } else if *v == largestSize {
                largestCount+=1;
            }
        }
        println!("{:?}", sums);
        return largestCount;
    }
}

pub fn sum_of_digits(n: i32) -> i32 {
    if n >= 10 {
        return (n%10) + sum_of_digits(n/10);
    } else {
        return n;
    }
	
}
