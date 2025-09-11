function canJump(nums: number[]): boolean {
  if (nums.length === 1) {
    return true;
  }
  let maxJumps = 0;
  for (let i: number = 0; i < nums.length - 1; i++) {
    maxJumps = Math.max(maxJumps, nums[i]);
    if (maxJumps <= 0) {
      return false;
    }
    maxJumps--;
  }
  return true;
}
