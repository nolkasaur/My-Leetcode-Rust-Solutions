// https://leetcode.com/problems/time-needed-to-buy-tickets/
// 0 ms, 2.1 MB

impl Solution {
    pub fn time_required_to_buy(mut tickets: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        loop {
            let mut x = 0;
            while  x < tickets.len() as i32 {
                if tickets[x as usize] != 0 {
                    res += 1;
                    tickets[x as usize] -= 1;
                }
                if x == k &&  tickets[x as usize] == 0 {
                    return res
                }
                x += 1;
            }
        }
        return -1;
    }
}
