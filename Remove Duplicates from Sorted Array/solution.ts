function removeDuplicates(nums: number[]): number {
  let k: number = 1;
  let j: number = 0;
  for (let i: number = 0; i < nums.length; i++) {
    if (nums[i] !== nums[j]) {
      j++;
      nums[j] = nums[i];
      k++;
    }
  }
  return k;
}
