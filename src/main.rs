mod leet_code;

fn main () {
    let m = 3;
    let n = 3;
    let mut nums1 = vec![1,2,3,0,0,0];
    let mut nums2 = vec![1,5,6];
    leet_code::solution::merge(&mut nums1, m, &mut nums2, n);
    println!("nums1 is {:?}", nums1);
    println!("nums2 is {:?}", nums2);
}