pub fn fabonaci(n: i32) -> i32 {
    let mut left;
    let mut right = 0;
    let mut result = 1;
    for _ in 0..n {
        left= right;
        right = result;
        result = left + right;
    }
    return result;
}