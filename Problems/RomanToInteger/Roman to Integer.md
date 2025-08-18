---
created: 2025-08-18
modified: 
completed: true
leetcode-index: 13
link: https://leetcode.com/problems/roman-to-integer
difficulty: Easy
tags:
  - leetcode/hash-table
  - leetcode/math
  - leetcode/string
  - leetcode/problem
---
# Roman to Integer

## Problem Statement
Roman numerals are represented by seven different symbols: `I`, `V`, `X`, `L`, `C`, `D` and `M`.

`
Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000`

For example, `2` is written as `II` in Roman numeral, just two ones added together. `12` is written as `XII`, which is simply `X + II`. The number `27` is written as `XXVII`, which is `XX + V + II`.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not `IIII`. Instead, the number four is written as `IV`. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as `IX`. There are six instances where subtraction is used:

	
- `I` can be placed before `V` (5) and `X` (10) to make 4 and 9. 
	
- `X` can be placed before `L` (50) and `C` (100) to make 40 and 90. 
	
- `C` can be placed before `D` (500) and `M` (1000) to make 400 and 900.

Given a roman numeral, convert it to an integer.

 

>[!Example]+ Example 1
>**Input**: `s = "III"`
>**Output**: `3`
>**Explanation**:
>III = 3. 

>[!Example]+ Example 2
>**Input**: `s = "LVIII"`
>**Output**: `58`
>**Explanation**:
>L = 50, V= 5, III = 3. 

>[!Example]+ Example 3
>**Input**: `s = "MCMXCIV"`
>**Output**: `1994`
>**Explanation**:
>M = 1000, CM = 900, XC = 90 and IV = 4. 

>[!warning]+ Constraints
>- `1 <= s.length <= 15`
>
>- `s` contains only the characters `('I', 'V', 'X', 'L', 'C', 'D', 'M')`.
>
>- It is guaranteed that `s` is a valid roman numeral in the range `[1, 3999]`.
## Hints
>[!Hint]- Hint 1
>Problem is simpler to solve by working the string from back to front and using a map.
## Approach

- Loop through the chars of the string, as long as we are not on the last char and the next char is greater than the current one we can subtract it because of how roman numerals are.
- else just add it to the sum
## Solution

```go
# Solution
package goenv

func _(s string) int {
	numeralsMap := map[byte]int{
		'I': 1,
		'V': 5,
		'X': 10,
		'L': 50,
		'C': 100,
		'D': 500,
		'M': 1000,
	}
	result := 0
	for i := range s {
		if  i != len(s)-1 && numeralsMap[s[i]] < numeralsMap[s[i+1]]  {
			continue
		}
		if i != 0 && numeralsMap[s[i]] > numeralsMap[s[i-1]]  {
			result += numeralsMap[s[i]] - numeralsMap[s[i-1]]
		} else {
			result += numeralsMap[s[i]]
		}

	}
	return result
}
```

## Complexity Analysis

- Time complexity: $$O(n)$$
- Space complexity: $$O(1)$$

## Reflections
-  First iteration had 3 if statements and 2 loops, when 1 was needed, but I solved it at the end