pub fn fabonaci(n: i32) -> i32 {
    let mut left;
    let mut right = 0;
    let mut result = 1;
    for i in 0..n {
        if i > 0 {
            left= right;
            right = result;
            result = left + right;
        }
    }
    return result;
}