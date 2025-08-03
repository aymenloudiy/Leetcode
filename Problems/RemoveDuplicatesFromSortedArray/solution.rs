impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k: i32 = 1;
        let mut i: usize = 0;
        let mut j: usize = 1;
        while j <= nums.len() - 1 {
            if nums[i] != nums[j] {
                k += 1;
                i += 1;
                nums[i] = nums[j];
            }
            j += 1;
        }
        return k;
        // nums.dedup();
        // nums.len() as i32
    }
}
