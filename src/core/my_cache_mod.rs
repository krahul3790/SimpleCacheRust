
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

pub struct Cache<K: Eq + Hash, V: Clone> {
    map: HashMap<K, V>,
    usage_order: VecDeque<K>,
    capacity,
}

impl<K: Eq + Hash, V: Clone> Cache<K, V> {
    pub fn new() -> Self {
        Cache {
            map: HashMap::new(),
            usage_order: VecDeque::new(),
            capacity: 10
        }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Cache {
            map: HashMap::new(),
            usage_order: VecDeque::new(),
            capacity,
        }
    }

    pub fn size(&self) -> usize {
        self.map.len()
    }

    pub fn put(&mut self, key: K, val: V) {
        self.map.insert(key, val);
    }

    pub fn get(&self, key: K) -> Option<&V> {
        if self.map.contains_key(&key) {
            self.map.get(&key)
        } else {
            None
        }
    }

    pub fn delete_key(&mut self, key: K) {
        if self.map.contains_key(&key) {
            self.map.remove(&key);
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }
}
