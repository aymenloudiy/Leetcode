const removeDuplicates = (nums: number[]): number => {
  let ans: number = 1;
  let counter: number = 1;
  for (let i: number = 1; i < nums.length; i++) {
    if (nums[ans - 1] == nums[i]) {
      if (counter == 2) {
        continue;
      }
      nums[ans++] = nums[i];
      counter++;
    } else {
      nums[ans++] = nums[i];
      counter = 1;
    }
  }
  return ans;
};
