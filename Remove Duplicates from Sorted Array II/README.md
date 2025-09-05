---
created: 2025-09-05
modified:
completed: true
leetcode-index: 80
link: https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii
difficulty: Medium
tags:
  - leetcode/array
  - leetcode/two-pointers
  - leetcode/problem
---
# Remove Duplicates from Sorted Array II

## Problem Statement
Given an integer array `nums` sorted in non-decreasing order, remove some duplicates <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> such that each unique element appears at most twice. The relative order of the elements should be kept the same.

Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array `nums`. More formally, if there are `k` elements after removing the duplicates, then the first `k` elements of `nums` should hold the final result. It does not matter what you leave beyond the first `k` elements.

Return `k`* after placing the final result in the first *`k`* slots of *`nums`.

Do not allocate extra space for another array. You must do this by modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> with O(1) extra memory.

Custom Judge:

The judge will test your solution with the following code:

`
int[] nums = [...]; // Input array
int[] expectedNums = [...]; // The expected answer with correct length

int k = removeDuplicates(nums); // Calls your implementation

assert k == expectedNums.length;
for (int i = 0; i < k; i++) {
    assert nums[i] == expectedNums[i];
}
`

If all assertions pass, then your solution will be accepted.

 

>[!Example]+ Example 1
>**Input**: `nums = [1,1,1,2,2,3]`
>**Output**: `5, nums = [1,1,2,2,3,_]`
>**Explanation**:
>Your function should return k = 5, with the first five elements of nums being 1, 1, 2, 2 and 3 respectively. It does not matter what you leave beyond the returned k (hence they are underscores). 

>[!Example]+ Example 2
>**Input**: `nums = [0,0,1,1,1,1,2,3,3]`
>**Output**: `7, nums = [0,0,1,1,2,3,3,_,_]`
>**Explanation**:
>Your function should return k = 7, with the first seven elements of nums being 0, 0, 1, 1, 2, 3 and 3 respectively. It does not matter what you leave beyond the returned k (hence they are underscores). 

>[!warning]+ Constraints
>- `1 <= nums.length <= 3 * 10^4`
>
>- `-10^4 <= nums[i] <= 10^4`
>
>- `nums` is sorted in non-decreasing order.
## Hints
No hints available.
## Approach

- The fact that this solution works is giving me anxiety, I hate it.
## Solution

```ts
# Solution
function removeDuplicates(nums: number[]): number {
    let j: number = 0
    for (let i: number = 0; i < nums.length; i++) {
        if (nums[j-2] !== nums[i]){
            nums[j] = nums[i]
            j++
        }
    }
    return j
};
```

## Complexity Analysis

- Time complexity: $$O(n)$$
- Space complexity: $$O(1)$$

## Reflections
- I hate JS