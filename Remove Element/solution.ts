function removeElement(nums: number[], val: number): number {
  if (nums.length == 1 && nums[0] != val) {
    return nums.length;
  } else if (nums.length == 1 && nums[0] == val) {
    return 0;
  }
  let result: number = 0;
  for (let i: number = 0, j: number = nums.length - 1; i <= j; i++) {
    while (nums[j] == val) {
      j--;
      result++;
    }
    if (j < i) {
      break;
    }
    if (nums[i] == val) {
      let temp: number = 0;
      temp = nums[j];
      nums[j] = nums[i];
      nums[i] = temp;
    }
  }
  return nums.length - result;
  // let result:number = 0
  // for (let i:number = 0;i <nums.length;i++){
  //     if (nums[i] == val) {
  //         nums[i] = 101
  //         result ++
  //     }
  // }
  // nums.sort((a,b)=>a-b)
  // return nums.length - result
}
