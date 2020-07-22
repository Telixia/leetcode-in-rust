// Leetcode 1486
pub fn xor_operation(n: i32, start: i32) -> i32 {
    let mut res: i32 = 0;
    
    for i in 0..n {
        res ^= (start + 2 * i);
    }
    
    res
}
