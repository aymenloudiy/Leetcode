function removeDuplicates(nums: number[]): number {
  let j: number = 0;
  for (let i: number = 0; i < nums.length; i++) {
    if (nums[j - 2] !== nums[i]) {
      nums[j] = nums[i];
      j++;
    }
  }
  return j;
}
