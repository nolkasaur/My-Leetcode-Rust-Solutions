// https://leetcode.com/problems/maximum-number-of-words-you-can-type/
// 2 ms, 2 MB

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let split = text.split(' ');
        let mut res = 0;
        'outer: for x in split {
            for y in x.chars() {
                for z in broken_letters.chars() {
                    if y == z {
                        continue 'outer;
                    }
                }
            }
            res+=1;
        }
        return res as i32;
    }
}
