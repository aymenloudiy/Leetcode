---
created: 2025-09-13
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

- 
## Solution

```ts
# Solution
function hIndex(citations: number[]): number {
    let hIndex: number = 0
    citations.sort((a, b) => b - a)
    for (let i: number = 0; i < citations.length; i++) {
        if (citations[i] < i + 1) {
            return hIndex
        }
        hIndex += 1
    }
    return hIndex

};
```

## Complexity Analysis

- Time complexity: 
- Space complexity: 

## Reflections
- 