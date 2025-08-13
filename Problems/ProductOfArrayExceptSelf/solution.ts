function productExceptSelf(nums: number[]): number[] {
  let answer: number[] = [];
  let rightSum: number[] = [];
  let product: number = 1;
  for (let j: number = nums.length - 1; j > -1; j--) {
    rightSum.push(product);
    product *= nums[j];
  }
  product = 1;
  let k: number = rightSum.length - 1;
  for (let i: number = 0; i < nums.length; i++) {
    answer.push(product * rightSum[k]);
    product *= nums[i];
    k--;
  }
  return answer;
}
