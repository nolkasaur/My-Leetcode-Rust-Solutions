// https://leetcode.com/problems/sort-array-by-parity-ii/
// 8 ms, 2.2 MB

impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let mut x = 0;
        while x < nums.len() {
            if x  % 2 == 0 {
                if nums[x] % 2 != 0 {
                    let mut y = x+1;
                    while y < nums.len() {
                        if nums[y] % 2 == 0 {
                            let aux = nums[x];
                            nums[x] = nums[y];
                            nums[y] = aux;
                            break;
                        }
                        y+=1;
                    }
                }
            } else {
                if nums[x] % 2 == 0 {
                    let mut y = x+1;
                    while y < nums.len() {
                        if nums[y] % 2 != 0 {
                            let aux = nums[x];
                            nums[x] = nums[y];
                            nums[y] = aux;
                            break;
                        }
                        y+=1;
                    }
                }
            }
            x+=1;
        }
        return nums;
    }
}
