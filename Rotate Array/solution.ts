/**
 Do not return anything, modify nums in-place instead.
 */
function rotate(nums: number[], k: number): void {
  if (k > nums.length) {
    k = k % nums.length;
  }
  nums.reverse();
  let nums1 = nums.splice(k, nums.length).reverse();
  nums.reverse().push(...nums1);
}
