---
created: 2025-08-12
modified: 
completed: true
leetcode-index: 380
link: https://leetcode.com/problems/insert-delete-getrandom-o1
difficulty: Medium
tags:
  - leetcode/array
  - leetcode/hash-table
  - leetcode/math
  - leetcode/design
  - leetcode/randomized
  - leetcode/problem
---
# Insert Delete GetRandom O(1)

## Problem Statement
Implement the `RandomizedSet` class:

	
- `RandomizedSet()` Initializes the `RandomizedSet` object.
	
- `bool insert(int val)` Inserts an item `val` into the set if not present. Returns `true` if the item was not present, `false` otherwise.
	
- `bool remove(int val)` Removes an item `val` from the set if present. Returns `true` if the item was present, `false` otherwise.
	
- `int getRandom()` Returns a random element from the current set of elements (it's guaranteed that at least one element exists when this method is called). Each element must have the <b>same probability</b> of being returned.

You must implement the functions of the class such that each function works in average `O(1)` time complexity.

 

>[!Example]+ Example 1

>[!warning]+ Constraints
>- `-2^31 <= val <= 2^31 - 1`
>
>- At most `2 * ``10^5` calls will be made to `insert`, `remove`, and `getRandom`.
>
>- There will be at least one element in the data structure when `getRandom` is called.
## Hints
No hints available.
## Approach

- Thank you random person in the discussion telling me about swapping last element for O(1) time.
- The only funny business is needing a hash map and a vector, and the swap in the vector.
## Solution

```rust
# Solution
use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;

struct RandomizedSet {
    hashmap: HashMap<i32, i32>,
    elements: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
            elements: Vec::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.hashmap.contains_key(&val) {
            false
        } else {
            self.hashmap.insert(val, self.elements.len() as i32);
            self.elements.push(val);
            true
        }
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if let Some(&index) = self.hashmap.get(&val) {
            let last_element = *self.elements.last().unwrap();
            let last_index = self.elements.len() as i32 - 1;

            self.hashmap.insert(last_element, index);
            self.elements.swap(index as usize, last_index as usize);
            self.hashmap.remove(&val);
            self.elements.pop();

            true
        } else {
            false
        }
    }
    
    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        match self.elements.choose(&mut rng) {
            Some(i) => *i,
            None => panic!("Cannot get a random element from an empty set!"),
        }
    }
}
```

## Complexity Analysis

- Time complexity: $$O(1)$$
- Space complexity: $$O(1)$$

## Reflections
- Rust is a pain, spent more time fighting the compiler than doing the problem