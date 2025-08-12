use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

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
