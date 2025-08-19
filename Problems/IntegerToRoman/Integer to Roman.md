---
created: 2025-08-19
modified: 
completed: true
leetcode-index: 12
link: https://leetcode.com/problems/integer-to-roman
difficulty: Medium
tags:
  - leetcode/hash-table
  - leetcode/math
  - leetcode/string
  - leetcode/problem
---
# Integer to Roman

## Problem Statement
Seven different symbols represent Roman numerals with the following values:

<table>
	<thead>
		<tr>
			<th>Symbol</th>
			<th>Value</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td>I</td>
			<td>1</td>
		</tr>
		<tr>
			<td>V</td>
			<td>5</td>
		</tr>
		<tr>
			<td>X</td>
			<td>10</td>
		</tr>
		<tr>
			<td>L</td>
			<td>50</td>
		</tr>
		<tr>
			<td>C</td>
			<td>100</td>
		</tr>
		<tr>
			<td>D</td>
			<td>500</td>
		</tr>
		<tr>
			<td>M</td>
			<td>1000</td>
		</tr>
	</tbody>
</table>

Roman numerals are formed by appending the conversions of decimal place values from highest to lowest. Converting a decimal place value into a Roman numeral has the following rules:

	
- If the value does not start with 4 or 9, select the symbol of the maximal value that can be subtracted from the input, append that symbol to the result, subtract its value, and convert the remainder to a Roman numeral.
	
- If the value starts with 4 or 9 use the subtractive form representing one symbol subtracted from the following symbol, for example, 4 is 1 (`I`) less than 5 (`V`): `IV` and 9 is 1 (`I`) less than 10 (`X`): `IX`. Only the following subtractive forms are used: 4 (`IV`), 9 (`IX`), 40 (`XL`), 90 (`XC`), 400 (`CD`) and 900 (`CM`).
	
- Only powers of 10 (`I`, `X`, `C`, `M`) can be appended consecutively at most 3 times to represent multiples of 10. You cannot append 5 (`V`), 50 (`L`), or 500 (`D`) multiple times. If you need to append a symbol 4 times use the subtractive form.

Given an integer, convert it to a Roman numeral.

 

>[!Example]+ Example 1
>**Input**: `num = 3749`
>**Output**: `"MMMDCCXLIX"`
>**Explanation**:
>` 3000 = MMM as 1000 (M) + 1000 (M) + 1000 (M)  700 = DCC as 500 (D) + 100 (C) + 100 (C)   40 = XL as 10 (X) less of 50 (L)    9 = IX as 1 (I) less of 10 (X) Note: 49 is not 1 (I) less of 50 (L) because the conversion is based on decimal places ` 

>[!Example]+ Example 2
>**Input**: `num = 58`
>**Output**: `"LVIII"`
>**Explanation**:
>` 50 = L  8 = VIII ` 

>[!Example]+ Example 3
>**Input**: `num = 1994`
>**Output**: `"MCMXCIV"`
>**Explanation**:
>` 1000 = M  900 = CM   90 = XC    4 = IV ` 

>[!warning]+ Constraints
>- `1 <= num <= 3999`
## Hints
No hints available.
## Approach

- Split the number to thousands, hundreads etc..

- For each new number pull its roman character for the hashmap and append it to the solution
## Solution

```rust
# Solution
use std::collections::HashMap;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {


        let roman_map: HashMap<i32, &str> = HashMap::from([
        (1, "I"),
        (2, "II"),
        (3, "III"),
        (4, "IV"),
        (5, "V"),
        (6, "VI"),
        (7, "VII"),
        (8, "VIII"),
        (9, "IX"),
        (10, "X"),
        (20, "XX"),
        (30, "XXX"),
        (40, "XL"),
        (50, "L"),
        (60, "LX"),
        (70, "LXX"),
        (80, "LXXX"),
        (90, "XC"),
        (100, "C"),
        (200, "CC"),
        (300, "CCC"),
        (400, "CD"),
        (500, "D"),
        (600, "DC"),
        (700, "DCC"),
        (800, "DCCC"),
        (900, "CM"),
        (1000, "M"),
        (2000, "MM"),
        (3000, "MMM"),
    ]);
    let num_len = num.checked_ilog10().unwrap_or(0) + 1;
    let mut answer: String = "".to_string();
    for i in (0..num_len).rev() {
        let digit = ((num / 10_i32.pow(i)) % 10) * 10_i32.pow(i);
        if digit == 0 {
            continue;
        }
        answer += roman_map.get(&digit).unwrap();
    }
    answer.to_string()

    }
}
//this was my initial solution but one of the solutions I found with brute force was funny so I decided to add it, it is also faster and uses less space than mine.
    const ONES: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
const TENS: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
const HUNDREADS: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
const THOUSANDS: [&str; 4] = ["", "M", "MM", "MMM"];
    format!(
        "{}{}{}{}",
        THOUSANDS[(num / 1000 % 10) as usize],
        HUNDREADS[(num / 100 % 10) as usize],
        TENS[(num / 10 % 10) as usize],
        ONES[(num % 10) as usize],
    )

//another way of doing it I learned since this is for learning the greedy approach
    let values = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
        let mut result = String::new();

    for (value, symbol) in &values {
        while num >= *value {
            result.push_str(symbol);
            num -= value;
        }
    }

    result
```

## Complexity Analysis

- Time complexity: $$O(1)$$
- Space complexity: $$O(1)$$

## Reflections
- So many ways to do 1 thing.