---
created: 2025-09-02
modified:
completed: true
leetcode-index: 88
link: https://leetcode.com/problems/merge-sorted-array
difficulty: Easy
tags:
  - leetcode/array
  - leetcode/two-pointers
  - leetcode/sorting
  - leetcode/problem
---
# Merge Sorted Array

## Problem Statement
You are given two integer arrays `nums1` and `nums2`, sorted in non-decreasing order, and two integers `m` and `n`, representing the number of elements in `nums1` and `nums2` respectively.

Merge `nums1` and `nums2` into a single array sorted in non-decreasing order.

The final sorted array should not be returned by the function, but instead be *stored inside the array *`nums1`. To accommodate this, `nums1` has a length of `m + n`, where the first `m` elements denote the elements that should be merged, and the last `n` elements are set to `0` and should be ignored. `nums2` has a length of `n`.

 

>[!Example]+ Example 1
>**Input**: `nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3`
>**Output**: `[1,2,2,3,5,6]`
>**Explanation**:
>The arrays we are merging are [1,2,3] and [2,5,6]. The result of the merge is [<u>1</u>,<u>2</u>,2,<u>3</u>,5,6] with the underlined elements coming from nums1. 

>[!Example]+ Example 2
>**Input**: `nums1 = [1], m = 1, nums2 = [], n = 0`
>**Output**: `[1]`
>**Explanation**:
>The arrays we are merging are [1] and []. The result of the merge is [1]. 

>[!Example]+ Example 3
>**Input**: `nums1 = [0], m = 0, nums2 = [1], n = 1`
>**Output**: `[1]`
>**Explanation**:
>The arrays we are merging are [] and [1]. The result of the merge is [1]. Note that because m = 0, there are no elements in nums1. The 0 is only there to ensure the merge result can fit in nums1. 

>[!warning]+ Constraints
>- `nums1.length == m + n`
>
>- `nums2.length == n`
>
>- `0 <= m, n <= 200`
>
>- `1 <= m + n <= 200`
>
>- `-10^9 <= nums1[i], nums2[j] <= 10^9`
>
>
>
>
>
>
>
>
>Follow up: Can you come up with an algorithm that runs in `O(m + n)` time?
## Hints
>[!Hint]- Hint 1
>You can easily solve this problem if you simply think about two elements at a time rather than two arrays. We know that each of the individual arrays is sorted. What we don't know is how they will intertwine. Can we take a local decision and arrive at an optimal solution?

>[!Hint]- Hint 2
>If you simply consider one element each at a time from the two arrays and make a decision and proceed accordingly, you will arrive at the optimal solution.
## Approach

- Remove the extra zeros
- Merge the arrays
- Sort the array
## Solution

```ts
# Solution
function merge(nums1: number[], m: number, nums2: number[], n: number): void {
    nums1.splice(m,n);
    nums1.push(...nums2);
    nums1.sort((a,b)=>a-b);
};
```

## Complexity Analysis

- Time complexity: $$O((m + n) log(m + n))$$
- Space complexity: $$O(1)$$

## Reflections
Could have used the extra space given in num1 array with 3 pointers to move the elements around, for better time complexity.