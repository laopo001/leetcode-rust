use crate::base::Solution;

impl Solution {
    #[allow(while_true)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut len = m + n - 1;
        let mut j = n - 1;
        let mut i = m - 1;
        while true {
            if i < m && i >= 0 && j < n && j >= 0 {
                if nums1[i as usize] > nums2[j as usize] {
                    nums1[len as usize] = nums1[i as usize];
                    len -= 1;
                    i -= 1;
                } else {
                    nums1[len as usize] = nums2[j as usize];
                    len -= 1;
                    j -= 1;
                }
            } else if i < m && i >= 0 {
                nums1[len as usize] = nums1[i as usize];
                len -= 1;
                i -= 1;
            } else if j < n && j >= 0 {
                nums1[len as usize] = nums2[j as usize];
                len -= 1;
                j -= 1;
            }
            if len < 0 {
                break;
            }
        }
    }
}
