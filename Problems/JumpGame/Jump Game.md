---
created: 2025-08-09
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

- Keep looking for furthest value I can reach each step.

- If I surpass that value, I can't proceed.

- If I don't I reach the end of the vector.
## Solution

```rust []
# Solution
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut farthest = 0;
        for i in 0..nums.len(){
            if farthest < i {
                return false;
            }
            farthest = farthest.max(i + nums[i] as usize);
        }
        true
    }
}
```

## Complexity Analysis

- Time complexity: $$O(n)$$
- Space complexity: $$O(1)$$

## Reflections
- Needed help with the logic of this problem, I should have spent more time and instead of rushing to learn how to do it, since it's very simple