// Leetcode #1470
pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut res = Vec::new();
    let mut a = 0;
    let mut b = n;
    let mut i = 0;
    
    while i < nums.len() {
        res.push(nums[a as usize]);
        res.push(nums[b as usize]);
        
        a += 1;
        b += 1;
        i += 2;
    }
    
    res
}
