---
created: 2025-08-21
modified: 
completed: true
leetcode-index: 58
link: https://leetcode.com/problems/length-of-last-word
difficulty: Easy
tags:
  - leetcode/string
  - leetcode/problem
---
# Length of Last Word

## Problem Statement
Given a string `s` consisting of words and spaces, return *the length of the last word in the string.*

A word is a maximal <span data-keyword="substring-nonempty">substring</span> consisting of non-space characters only.

 

>[!Example]+ Example 1
>**Input**: `s = "Hello World"`
>**Output**: `5`
>**Explanation**:
>The last word is "World" with length 5. 

>[!Example]+ Example 2
>**Input**: `s = "   fly me   to   the moon  "`
>**Output**: `4`
>**Explanation**:
>The last word is "moon" with length 4. 

>[!Example]+ Example 3
>**Input**: `s = "luffy is still joyboy"`
>**Output**: `6`
>**Explanation**:
>The last word is "joyboy" with length 6. 

>[!warning]+ Constraints
>- `1 <= s.length <= 10^4`
>
>- `s` consists of only English letters and spaces `' '`.
>
>- There will be at least one word in `s`.
## Hints
No hints available.
## Approach

- If it there were no array method, Id have to loop through the string
- Split by empty space
- Count the length of the last non empty space element in the array and return it
- This is easier
## Solution

```ts
# Solution
function lengthOfLastWord(s: string): number {
    return s.trim().split(" ").pop().length
};
```

## Complexity Analysis

- Time complexity: $$O(n)$$
- Space complexity: $$O(1)$$

## Reflections
- 