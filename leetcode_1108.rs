// Leetcode 1108
pub fn defang_i_paddr(address: String) -> String {
    address.replace(".", "[.]")
}

// There's another way to solve this using Map. I am not well-versed with Map in Rust atm. I will update the answer once I get comfortable with it