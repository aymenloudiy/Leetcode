---
created: 2025-08-05
modified: 
completed: true
leetcode-index: 169
link: https://leetcode.com/problems/majority-element
difficulty: Easy
tags:
  - leetcode/array
  - leetcode/hash-table
  - leetcode/divide-and-conquer
  - leetcode/sorting
  - leetcode/counting
  - leetcode/problem
---
# Majority Element

## Problem Statement
Given an array `nums` of size `n`, return *the majority element*.

The majority element is the element that appears more than `&lfloor;n / 2&rfloor;` times. You may assume that the majority element always exists in the array.

 

>[!Example]+ Example 1
>**Input**: `nums = [3,2,3]`
>**Output**: `3
`

>[!Example]+ Example 2
>**Input**: `nums = [2,2,1,1,1,2,2]`
>**Output**: `2
`

>[!warning]+ Constraints
>- `n == nums.length`
>
>- `1 <= n <= 5 * 10^4`
>
>- `-10^9 <= nums[i] <= 10^9`

>[!Todo]- Follow Up
>Could you solve the problem in linear time and in `O(1)` space?
## Hints
No hints available.
## Approach

- Follow the Boyer-moore majority vote algorithm
## Solution

```go
# Solution
package main

func majorityElement(nums []int) int {
	count := 0
	curr := nums[0]
	for _, element := range nums {
		if element == curr {
			count += 1
		} else {
			if count == 0 {
				curr = element
				count = 1
			} else {
				count--
			}
		}
	}
	return curr
}
```

## Complexity Analysis

- Time complexity: $$O(n)$$
- Space complexity: $$O(1)$$

## Reflections
-  I remember solving this in the past