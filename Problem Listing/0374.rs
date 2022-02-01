// https://leetcode.com/problems/guess-number-higher-or-lower/
// 0 ms, 2.6 MB

use rand::Rng;

/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut lower: i64 = 0;
        let mut upper: i64 = n as i64;
        loop {
            if lower == upper { return lower as i32 }
            let mut rng = rand::thread_rng();
            let g = rng.gen_range(lower, upper+1);
            if guess(g as i32) == 0 { return g as i32 }
            else if guess(g as i32) == 1 { lower = g }
            else { upper = g }
        }
    }
}
