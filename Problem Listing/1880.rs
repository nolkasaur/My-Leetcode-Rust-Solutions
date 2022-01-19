// https://leetcode.com/problems/check-if-word-equals-summation-of-two-words/
// 0 ms, 2.1 MB

impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        return to_num(first_word) + to_num(second_word) == to_num(target_word);
    }
}

pub fn to_num(s: String) -> i32 {
    let mut strNr = String::from("");
    for x in s.chars() {
        let n = match x {
            'a' => '0',
            'b' => '1',
            'c' => '2',
            'd' => '3',
            'e' => '4',
            'f' => '5',
            'g' => '6',
            'h' => '7',
            'i' => '8',
            'j' => '9',
              _ => 'f',
        };
        strNr.push(n);
    }
    let res: i32 = strNr.trim().parse().unwrap();
    return res;
}
