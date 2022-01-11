// https://leetcode.com/problems/defanging-an-ip-address/
// 0 ms, 2 MB

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        return address.replace(".", "[.]");
    }
}
