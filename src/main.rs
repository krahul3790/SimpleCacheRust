// Define type for your cache.

use crate::core::my_cache_mod::Cache;

mod core;

/**
Stateful Entity :-

Cache

Operations on Cache entity
1. put key, value.
2. get key, value
3. remove key, value
*/

fn main() {
    // Build 1_000_000_000 keys and values.
    let mut my_cache = Cache::new();

    println!("Starting to build Cache....");
    for num in 1..1_000_000 {
        my_cache.put(num, num);
    }

    println!("Building cache complete!!");
}

#[cfg(test)]
mod tests {

    use super::core::my_cache_mod::*;

    #[test]
    fn verify_cache_creation() {
        let my_cache: Cache<&str, &str> = Cache::new();
        assert_eq!(0, my_cache.size());
    }

    #[test]
    fn verify_cache_put_works() {
        let mut my_cache = Cache::new();

        my_cache.put(String::from("Rahul"), String::from("Kulkarni"));

        assert_eq!(1, my_cache.size());
        assert_eq!("Kulkarni", my_cache.get(&String::from("Rahul")).unwrap());
    }

    #[test]
    fn verify_cache_deletion_works() {
        let mut my_cache = Cache::new();

        my_cache.put(1, 10);
        my_cache.put(2, 20);
        my_cache.put(3, 30);

        assert_eq!(10, *my_cache.get(&1).unwrap());

        my_cache.delete_key(1);

        assert_eq!(None, my_cache.get(&1));
    }

    #[test]
    fn verify_least_recently_used_element_is_evicted() {
        let mut my_cache = Cache::new_with_capacity(3);

        my_cache.put(1, 10);
        my_cache.put(2, 20);
        my_cache.put(3, 30);
        my_cache.put(4, 40);

        assert_eq!(false, my_cache.contains_key(&1));

    }
}
