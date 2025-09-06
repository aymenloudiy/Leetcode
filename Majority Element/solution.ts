function majorityElement(nums: number[]): number {
  let counter: number = 0;
  let answer: number = nums[0];
  for (let i: number = 0; i < nums.length; i++) {
    if (nums[i] == answer) {
      counter++;
    } else {
      counter--;
    }
    if (counter == 0) {
      answer = nums[i];
      counter = 1;
    }
  }
  return answer;
}
