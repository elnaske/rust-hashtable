use std::clone::Clone;
use std::hash::{DefaultHasher, Hash, Hasher};

// TODO:
// - make buckets linked lists instead of vectors
pub struct HashTable<K: Hash, V> {
    size: usize,
    buckets: Vec<Vec<(K, V)>>,
    count: usize,
}
impl<K: Clone + Hash, V: Clone> HashTable<K, V> {
    pub fn new(size: usize) -> Self {
        HashTable {
            size,
            buckets: vec![Vec::<(K, V)>::new(); size],
            count: 0,
        }
    }

    fn hash(&self, key: K) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let hash_result = s.finish() as usize; // maybe better conversion handling??
        hash_result % self.size
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hash_deterministic() {
        let ht = HashTable::<String, i32>::new(10);
        let res1 = ht.hash("a".to_string());
        let res2 = ht.hash("a".to_string());

        assert_eq!(res1, res2);
    }
}
