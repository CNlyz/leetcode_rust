// 给你两个按 非递减顺序 排列的整数数组 nums1 和 nums2，另有两个整数 m 和 n ，分别表示 nums1 和 nums2 中的元素数目。
// 请你 合并 nums2 到 nums1 中，使合并后的数组同样按 非递减顺序 排列。
// 注意：最终，合并后数组不应由函数返回，而是存储在数组 nums1 中。
// 为了应对这种情况，nums1 的初始长度为 m + n，其中前 m 个元素表示应合并的元素，后 n 个元素为 0 ，应忽略。nums2 的长度为 n 。
pub mod solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) -> Vec<i32> {
        let mut j = n - 1;
        loop {
            let mut i = m - 1;
            loop {
                let e1 = nums1[i as usize];
                let e2 = nums2[j as usize];
                if e1 > e2 {
                    nums1[i as usize] = e2;
                    nums2[j as usize] = e1;
                    break;
                }
                if i == 0 {
                    break;
                }
                i = i - 1;
            }
            if j == 0  {
                break nums1.to_vec();
            }
            j = j - 1;
        }
    }
}
