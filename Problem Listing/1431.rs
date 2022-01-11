// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
// 0 ms, 2.1 MB

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let len = candies.len();
        let greatest = *candies.iter().max().unwrap();
        let mut res: Vec<bool> = Vec::with_capacity(len);
        for x in 0..len {
            let total = candies[x]+extra_candies;
            if total >= greatest {
                res.push(true);
            } else {
                res.push(false);
            }
        }
        return res;
    }
}
