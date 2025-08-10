---
created: 2025-08-10
modified: 
completed: true
leetcode-index: 45
link: https://leetcode.com/problems/jump-game-ii
difficulty: Medium
tags:
  - leetcode/array
  - leetcode/dynamic-programming
  - leetcode/greedy
  - leetcode/problem
---
# Jump Game II

## Problem Statement
You are given a 0-indexed array of integers `nums` of length `n`. You are initially positioned at index 0.

Each element `nums[i]` represents the maximum length of a forward jump from index `i`. In other words, if you are at index `i`, you can jump to any index `(i + j)` where:

	
- `0 <= j <= nums[i]` and
	
- `i + j < n`

Return *the minimum number of jumps to reach index *`n - 1`. The test cases are generated such that you can reach index `n - 1`.

 

>[!Example]+ Example 1
>**Input**: `nums = [2,3,1,1,4]`
>**Output**: `2`
>**Explanation**:
>The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index. 

>[!Example]+ Example 2
>**Input**: `nums = [2,3,0,1,4]`
>**Output**: `2
`

>[!warning]+ Constraints
>- `1 <= nums.length <= 10^4`
>
>- `0 <= nums[i] <= 1000`
>
>- It's guaranteed that you can reach `nums[n - 1]`.
## Hints
No hints available.
## Approach

-  Keep count of how far you can go, if you reach it jump again until you get to the finish line.

## Solution

```ts
# Solution

function jump(nums: number[]): number {
    let jumps:number = 0;
    let farthest:number = 0;
    let pos:number = 0;
    for (let i:number = 0 ;i < nums.length -1; i++){
        farthest = Math.max(farthest,i+nums[i])
        if (pos == i){
            jumps++
            pos = farthest
        }
    }
    return jumps
};
```


## Complexity Analysis

- Time complexity: $$O(n)$$
- Space complexity: $$O(1)$$

## Reflections
- The nature of the problem forces a solution to be present else this would be hard.
