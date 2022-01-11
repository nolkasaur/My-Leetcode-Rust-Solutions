// https://leetcode.com/problems/defanging-an-ip-address/

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        return address.replace(".", "[.]");
    }
}
