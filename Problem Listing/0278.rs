// https://leetcode.com/problems/first-bad-version/
// 0 ms, 2.1 MB

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        if(n==1) { return n; }
        let mut leftBound: i32 = 1;
        let mut rightBound: i32 = n;
        let mut goodIndex = -1;
        loop {
            let mut index: i32 = leftBound + (rightBound - leftBound)/2;
            if self.isBadVersion(index) == true && (index-1 == goodIndex || index == 1) { return index; }
            else if self.isBadVersion(index) == false { goodIndex = index; leftBound = index+1; }
            else {
                rightBound = index-1;
            }
        }
        return -1;
    }
}
