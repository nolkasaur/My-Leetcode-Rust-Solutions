// https://leetcode.com/problems/number-of-rectangles-that-can-form-the-largest-square/
// 7 ms, 2.1 MB

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let len = rectangles.len();
        let mut squareArr:Vec<i32> = Vec::with_capacity(len);
        let mut largest = 0;
        for x in 0..len {
            let val = rectangles[x].iter().min().unwrap();
            squareArr.push(*val);
            if *val > largest { largest = *val; }
        }
        return squareArr.iter().filter(|x| **x == largest).count() as i32;
    }
}
