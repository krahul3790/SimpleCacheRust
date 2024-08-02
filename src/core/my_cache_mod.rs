
use std::collections::HashMap;
use std::hash::Hash;

pub struct Cache<K: Eq + Hash, V: Clone> {
    map: HashMap<K, V>,
}

impl<K: Eq + Hash, V: Clone> Cache<K, V> {
    pub fn new() -> Self {
        Cache {
            map: HashMap::new(),
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
}
