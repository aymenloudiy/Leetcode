---
created: 2025-08-01
modified: 
completed: true
leetcode-index: 88
link: https://leetcode.com/problems/merge-sorted-array
difficulty: Easy
tags:
  - leetcode/array
  - leetcode/two-pointers
  - leetcode/sorting
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

- Remove extra zeros from nums1 array.  
- Append nums2 to nums1.  
- Sort nums1.

There is no need to merge and sort in place in one operation unless the size of the area is actually huge.
## Solution

```ts
# Solution
/**
 Do not return anything, modify nums1 in-place instead.
 */
function merge(nums1: number[], m: number, nums2: number[], n: number): void {
    nums1.splice(m,n)
    nums1.push(...nums2)
    nums1.sort((a,b)=>{
        return a-b
    }) 
};
```

## Complexity Analysis

- Time complexity: $$O(Nlog(n))$$
- Space complexity: $$O(m+n)$$

## Reflections
-  Could have done it without using the sort and splice array methods but the constraints don't justify it.