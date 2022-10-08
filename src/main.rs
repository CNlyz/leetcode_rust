use std::time;
// mod merge;
mod fabonaci;

fn main () {
    // let m = 3;
    // let n = 3;
    // let mut nums1 = vec![1,2,3,0,0,0];
    // let mut nums2 = vec![1,5,6];
    // merge::merge(&mut nums1, m, &mut nums2, n);
    // println!("nums1 is {:?}", nums1);
    // println!("nums2 is {:?}", nums2);
    let time1 = time::SystemTime::now();
    let result = fabonaci::fabonaci(40);
    let time2 = time::SystemTime::now();
    println!("result is {result}");
    println!("cost time {:?}", time2.duration_since(time1));
}