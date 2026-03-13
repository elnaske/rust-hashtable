use std::clone::Clone;
use std::fmt::format;
use std::hash::{DefaultHasher, Hash, Hasher};

// TODO:
// - make buckets linked lists instead of vectors
pub struct HashTable<K: Hash, V> {
    buckets: Vec<Vec<(K, V)>>,
    count: usize,
}
impl<K: Clone + Hash + PartialEq, V: Clone> HashTable<K, V> {
    pub fn new(size: usize) -> Self {
        HashTable {
            buckets: vec![Vec::<(K, V)>::new(); size],
            count: 0,
        }
    }

    fn hash(&self, key: &K) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let hash_result = s.finish() as usize; // maybe better conversion handling??

        hash_result % self.buckets.len()
    }

    // pass by value or reference?
    pub fn put(&mut self, key: K, value: V) {
        let idx = self.hash(&key);
        let bucket = &mut self.buckets[idx];

        for (i, (k, _)) in bucket.iter().enumerate() {
            if k == &key {
                bucket[i] = (key, value);
                return;
            }
        }
        bucket.push((key, value));
        self.count += 1;
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let idx = self.hash(key);
        let bucket = &self.buckets[idx];

        // could probably be rewritten functionally
        for (k, v) in bucket {
            if k == key {
                return Some(v.clone());
            }
        }
        None
    }

    pub fn delete(&mut self, key: &K) -> Result<(), String> {
        todo!()
    }

    pub fn contains(&self, key: &K) -> bool {
        todo!()
    }

    fn resize(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hash_deterministic() {
        let ht = HashTable::<String, i32>::new(10);
        let res1 = ht.hash(&"a".to_string());
        let res2 = ht.hash(&"a".to_string());

        assert_eq!(res1, res2);
    }

    #[test]
    fn put_get() {
        let mut ht = HashTable::<String, i32>::new(10);
        let key = "a".to_string();
        let value = 1;

        ht.put(key, value);

        assert_eq!(value, ht.get(&"a".to_string()).unwrap());
    }
}
