// https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/
// 1 ms, 2.3 MB

use std::collections::VecDeque;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut qStudents: VecDeque<i32> = VecDeque::from(students);
        let mut qSandwiches: VecDeque<i32> = VecDeque::from(sandwiches);
        let mut gottem = 0;
        while(!qStudents.is_empty()) {
            let student = qStudents.pop_front().unwrap();
            let sandwich = qSandwiches.pop_front().unwrap();
            if student == sandwich {
                gottem = 0;
            } else {
                qStudents.push_back(student);
                qSandwiches.push_front(sandwich);
                gottem += 1;
            }
            if gottem == qStudents.len() {
                break;
            }
        }
        return qStudents.len() as i32;
    }
}
