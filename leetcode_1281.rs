// Leetcode 1281
pub fn subtract_product_and_sum(mut n: i32) -> i32 {
    let mut product: i32 = 1;
    let mut sum: i32 = 0;
    
    while n > 0 {
        let digit = n % 10;
        product *= digit;
        sum += digit;
        n /= 10;
    }
    
    product - sum
}
