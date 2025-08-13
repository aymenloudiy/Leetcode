---
created: 2025-08-13
modified: 
completed: true
leetcode-index: 238
link: https://leetcode.com/problems/product-of-array-except-self
difficulty: Medium
tags:
  - leetcode/array
  - leetcode/prefix-sum
  - leetcode/problem
---
# Product of Array Except Self

## Problem Statement
Given an integer array `nums`, return *an array* `answer` *such that* `answer[i]` *is equal to the product of all the elements of* `nums` *except* `nums[i]`.

The product of any prefix or suffix of `nums` is guaranteed to fit in a 32-bit integer.

You must write an algorithm that runs in `O(n)` time and without using the division operation.

 

>[!Example]+ Example 1
>**Input**: `nums = [1,2,3,4]`
>**Output**: `[24,12,8,6]
`

>[!Example]+ Example 2
>**Input**: `nums = [-1,1,0,-3,3]`
>**Output**: `[0,0,9,0,0]
`

>[!warning]+ Constraints
>- `2 <= nums.length <= 10^5`
>
>- `-30 <= nums[i] <= 30`
>
>- The input is generated such that `answer[i]` is guaranteed to fit in a 32-bit integer.
>
>
>
>
>
>
>
>
>Follow up: Can you solve the problem in `O(1)` extra space complexity? (The output array does not count as extra space for space complexity analysis.)
## Hints
>[!Hint]- Hint 1
>Think how you can efficiently utilize prefix and suffix products to calculate the product of all elements except self for each index. Can you pre-compute the prefix and suffix products in linear time to avoid redundant calculations?

>[!Hint]- Hint 2
>Can you minimize additional space usage by reusing memory or modifying the input array to store intermediate results?
## Approach

- Calculate the right product
- Multiply that by the left product
## Solution

```ts
# Solution
function productExceptSelf(nums: number[]): number[] {
    let answer: number[] = [];
    let rightSum: number[] = [];
    let product: number = 1;
    for (let j: number = nums.length - 1; j > -1; j--) {
        rightSum.push(product)
        product *= nums[j]
    }
    product = 1;
    let k: number = rightSum.length - 1
    for (let i: number = 0; i < nums.length; i++) {
        answer.push(product * rightSum[k])
        product *= nums[i]
        k--
    }
    return answer

};
```

## Complexity Analysis

- Time complexity: $$O(n)$$
- Space complexity: $$O(n)$$

## Reflections
- Needed help from discussion but no cheating was involved c: