use std::clone::Clone;
use std::hash::{DefaultHasher, Hash, Hasher};

// TODO:
// - make buckets linked lists instead of vectors
#[derive(Debug)]
pub struct HashTable<K: Hash, V> {
    buckets: Vec<Vec<(K, V)>>,
    count: usize,
}
impl<K: Clone + Hash + Eq, V: Clone> HashTable<K, V> {
    pub fn new(size: usize) -> Self {
        HashTable {
            buckets: vec![Vec::<(K, V)>::new(); size],
            count: 0,
        }
    }

    fn hash(&self, key: &K) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let hash_result = s.finish() as usize;

        hash_result % self.buckets.len()
    }

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

        if (self.count / self.buckets.len()) * 100 > 75 {
            self.resize();
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let idx = self.hash(key);
        let bucket = &self.buckets[idx];

        for (k, v) in bucket {
            if k == key {
                return Some(v.clone());
            }
        }
        None
    }

    pub fn get_or_else(&self, key: &K, default: V) -> V {
        match self.get(key) {
            Some(v) => v,
            None => default,
        }
    }

    pub fn delete(&mut self, key: &K) -> Result<(), &str> {
        let idx = self.hash(key);
        let bucket = &mut self.buckets[idx];
        
        match bucket.iter().position(|(k, _)| k == key) {
            Some(i) => {
                bucket.remove(i);
                self.count -= 1;
                Ok(())
            }
            None => Err("Key not found in hashtable"),
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        match self.get(key) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn keys(&self) -> Vec<K> {
        let mut keys: Vec<K> = Vec::new();

        for bucket in &self.buckets {
            for (key, _) in bucket {
                keys.push(key.clone());
            }
        }
        keys
    }

    pub fn len(&self) -> usize {
        self.count
    }

    fn resize(&mut self) {
        let new_size = self.buckets.len() * 2;
        let mut new_table = HashTable::<K, V>::new(new_size);

        for bucket in &mut self.buckets {
            for (key, value) in bucket.drain(..) {
                new_table.put(key, value);
            }
        }

        *self = new_table;
    }
}
impl<K: Clone + Hash + Eq, V: Clone + PartialEq> HashTable<K, V> {
    pub fn equals(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for bucket in &self.buckets {
            for (k, v) in bucket {
                match other.get(&k) {
                    Some(val) if val == *v => {},
                    _ => return false,
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{collections::HashSet, vec};

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

        ht.put(key.clone(), value);

        assert_eq!(value, ht.get(&key).unwrap());
    }

    #[test]
    fn delete_contains() {
        let mut ht = HashTable::<String, i32>::new(10);
        let key = "a".to_string();
        let value = 1;

        ht.put(key.clone(), value);
        assert!(ht.contains(&key));

        let _ = ht.delete(&key);
        assert!(!ht.contains(&key));
    }

    #[test]
    fn update() {
        let mut ht = HashTable::<String, i32>::new(10);
        let key = "a".to_string();
        let value = 1;

        ht.put(key.clone(), value);
        ht.put(key.clone(), value);
        assert!(ht.contains(&key));

        let contains_duplicate = {
            let mut seen = HashSet::new();
            
            let idx = ht.hash(&key);
            let bucket = &ht.buckets[idx];

            let mut res = false;

            for (k, _) in bucket {
                if !seen.insert(k) {
                    res = true;
                    break;
                }
            }
            res
        };

        assert!(!contains_duplicate);        
    }

    #[test]
    #[should_panic]
    fn delete_invalid() {
        let mut ht = HashTable::<String, i32>::new(10);
        ht.delete(&"a".to_string()).unwrap();
    }

    #[test]
    fn get_keys() {
        let mut ht = HashTable::<String, i32>::new(10);
        ht.put("a".to_string(), 1);
        ht.put("b".to_string(), 2);
        ht.put("b".to_string(), 3);
        ht.put("c".to_string(), 3);

        let mut keys = ht.keys();
        keys.sort();
        let reference = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(keys, reference);
    }

    #[test]
    fn resize() {
        let mut ht = HashTable::<String, i32>::new(2);

        ht.put("a".to_string(), 1);
        assert_eq!(ht.buckets.len(), 2);
        
        ht.put("b".to_string(), 2);
        assert_eq!(ht.buckets.len(), 4);
    }

    #[test]
    fn len() {
        let mut ht = HashTable::<String, i32>::new(10);
        ht.put("a".to_string(), 1);
        assert_eq!(ht.len(), 1);

        ht.put("b".to_string(), 1);
        assert_eq!(ht.len(), 2);
        
        ht.put("b".to_string(), 2);
        assert_eq!(ht.len(), 2);
        
        let _ = ht.delete(&"b".to_string());
        assert_eq!(ht.len(), 1);
    }

    #[test]
    fn equals() {
        let mut ht_1 = HashTable::<String, i32>::new(10);
        ht_1.put("a".to_string(), 1);
        
        let mut ht_2 = HashTable::<String, i32>::new(10);
        ht_2.put("b".to_string(), 1);

        assert!(!ht_1.equals(&ht_2));

        ht_2.put("a".to_string(), 2);
        let _ = ht_2.delete(&"b".to_string());

        assert!(!ht_1.equals(&ht_2));

        ht_2.put("a".to_string(), 1);

        assert!(ht_1.equals(&ht_2));
    }
}
