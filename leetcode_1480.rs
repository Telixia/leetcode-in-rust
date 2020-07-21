// Leetcode #1480
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut sum: i32 = 0;
    let mut res = Vec::new();
    
    for i in 0..nums.len() {
        sum += nums[i];
        res.push(sum);
    }
    
    res
}
