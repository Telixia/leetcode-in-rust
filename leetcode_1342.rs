// Leetcode 1342
pub fn number_of_steps (mut num: i32) -> i32 {
    let mut steps: i32 = 0;
    
    while num > 0 {
        num = if num % 2 == 0 { num / 2 } else { num - 1 };
        steps += 1;
    }
    
    steps
}
