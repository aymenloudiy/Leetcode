impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let length = nums.len();
        let mut k = k as usize % length;
        // nums.rotate_right(k)
        nums.reverse();
        nums[0..k].reverse();
        nums[k..].reverse();
    }
}
