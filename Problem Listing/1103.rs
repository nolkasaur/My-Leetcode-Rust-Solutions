// https://leetcode.com/problems/distribute-candies-to-people/
// 0 ms, 2.3 MB

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut candyLeft = candies;
        let mut vec = vec![0; num_people as usize];
        let mut count = 0;
        while candyLeft > 0 {
            if candyLeft >= count + 1 {
                vec[(count % num_people) as usize] += count + 1;
            } else {
                vec[(count % num_people) as usize] += candyLeft;
            }
            count += 1;
            candyLeft -= count;
        }
        return vec;
    }
}
