---
created: 2025-08-11
modified: 
completed: true
leetcode-index: 274
link: https://leetcode.com/problems/h-index
difficulty: Medium
tags:
  - leetcode/array
  - leetcode/sorting
  - leetcode/counting-sort
  - leetcode/problem
---
# H-Index

## Problem Statement
Given an array of integers `citations` where `citations[i]` is the number of citations a researcher received for their `i^th` paper, return *the researcher's h-index*.

According to the <a href="https://en.wikipedia.org/wiki/H-index" target="_blank">definition of h-index on Wikipedia</a>: The h-index is defined as the maximum value of `h` such that the given researcher has published at least `h` papers that have each been cited at least `h` times.

 

>[!Example]+ Example 1
>**Input**: `citations = [3,0,6,1,5]`
>**Output**: `3`
>**Explanation**:
>[3,0,6,1,5] means the researcher has 5 papers in total and each of them had received 3, 0, 6, 1, 5 citations respectively. Since the researcher has 3 papers with at least 3 citations each and the remaining two with no more than 3 citations each, their h-index is 3. 

>[!Example]+ Example 2
>**Input**: `citations = [1,3,1]`
>**Output**: `1
`

>[!warning]+ Constraints
>- `n == citations.length`
>
>- `1 <= n <= 5000`
>
>- `0 <= citations[i] <= 1000`
## Hints
>[!Hint]- Hint 1
>An easy approach is to sort the array first.

>[!Hint]- Hint 2
>What are the possible values of h-index?

>[!Hint]- Hint 3
>A faster approach is to use extra space.
## Approach

if f is the function that corresponds to the number of citations for each publication, we compute the h-index as follows: 

- First we order the values of f from the largest to the lowest value.

- Then, we look for the last position in which f is greater than or equal to the position (we call h this position)
## Solution

``` go
# Solution

func hIndex(citations []int) int {
	sort.Slice(citations, func(i, j int) bool {
		return citations[i] > citations[j]
	})
	fmt.Println(citations)
    max:=0
	for index, element := range citations {
		fmt.Println(element, index)
		if element <= index {
			return index
		}
        max+=1
	}
    return max
}
```

## Complexity Analysis

- Time complexity: $$O(nlog(n))$$
- Space complexity: $$O(1)$$

## Reflections
- Spent more time understanding H index, thank you wikipedia