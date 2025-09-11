---
created: 2025-09-10
modified:
completed: true
leetcode-index: 55
link: https://leetcode.com/problems/jump-game
difficulty: Medium
tags:
  - leetcode/array
  - leetcode/dynamic-programming
  - leetcode/greedy
  - leetcode/problem
---
# Jump Game

## Problem Statement
You are given an integer array `nums`. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.

Return `true`* if you can reach the last index, or *`false`* otherwise*.

 

>[!Example]+ Example 1
>**Input**: `nums = [2,3,1,1,4]`
>**Output**: `true`
>**Explanation**:
>Jump 1 step from index 0 to 1, then 3 steps to the last index. 

>[!Example]+ Example 2
>**Input**: `nums = [3,2,1,0,4]`
>**Output**: `false`
>**Explanation**:
>You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index. 

>[!warning]+ Constraints
>- `1 <= nums.length <= 10^4`
>
>- `0 <= nums[i] <= 10^5`
## Hints
No hints available.
## Approach

- 
## Solution

```ts
# Solution
function canJump(nums: number[]): boolean {
    if (nums.length === 1) {
        return true
    }
    let maxJumps = 0
    for (let i:number = 0; i < nums.length - 1; i++) {
        maxJumps = Math.max(maxJumps,nums[i])
        if (maxJumps <= 0) {
            return false
        }
        maxJumps--
    }
    return true
};
```

## Complexity Analysis

- Time complexity: $$O(N)$$
- Space complexity: $$O(1)$$

## Reflections
- 