---
created: 2025-08-04
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

- Start at the second index, and check if the previous element is the same as the current one.
- If it's the same element and the counter is 2 just move to the next iteration after incrementing the pointer.
- if the counter is not at 2 change the current element on the pointer to that of the loop's index, then increment the counter and the pointer by 1.
- If the elements are different same logic as the last point made, but reset the counter to 1 this time.
## Solution

```ts
# Solution
const removeDuplicates = (nums: number[]): number => {
let ans:number = 1;
let counter:number = 1;
for (let i:number = 1; i < nums.length; i++){
    if (nums[ans-1] == nums[i]){
        if (counter == 2){
            continue
        }
        nums[ans++] = nums[i]
        counter++
    }
    else {
        nums[ans++] = nums[i]
        counter = 1
    }
}
return ans
};


```

## Complexity Analysis

- Time complexity: $$O(n)$$
- Space complexity: $$O(1)$$

## Reflections
-  I need to practice more, this was supposed to be easy, but i struggled with the logic for too long, and needed to check others solutions to know what I did wrong, I was definitely on the right track but couldn't take the last step.