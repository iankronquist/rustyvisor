use alloc::arc::Arc;
use spin::RwLock;
use core::hash::{Hash, Hasher};
// Compiler warns about a field not exported in core.
#[allow(deprecated)]
use core::hash::SipHasher;
use collections::vec::Vec;
use core::mem;
use core::cmp;

enum HashMapMember<K: Hash + cmp::PartialEq, V> {
    None,
    Tombstone,
    Bucket(Bucket<K, V>),
}


struct Bucket<K: Hash + cmp::PartialEq, V> {
    key: K,
    value: Arc<V>,
}


pub struct HashMap<K: Hash + cmp::PartialEq, V> {
    count: usize,
    table: RwLock<Vec<HashMapMember<K, V>>>,
    rebalance_factor: usize,
}


impl<K: Hash + cmp::PartialEq, V> Bucket<K, V> {
    fn new(key: K, value: Arc<V>) -> Self {
        Bucket {
            key: key,
            value: value,
        }
    }
}


impl<K: Hash + cmp::PartialEq, V> HashMap<K, V> {
    pub fn new(size: usize) -> Self {
        let mut table = vec![];
        for _ in 0..size {
            table.push(HashMapMember::None);
        }
        HashMap {
            count: 0,
            table: RwLock::new(table),
            rebalance_factor: 4,
        }
    }


    fn should_resize(&self) -> bool {
        let size = self.table.read().len();
        (size - self.count) < (size / self.rebalance_factor)
    }


    fn rebalance(&mut self) {
        let mut table = self.table.write();
        let mut temp = HashMap::new(table.len() * 2);
        for entry in table.drain(1..) {
            match entry {
                HashMapMember::Bucket(b) => temp.insert_rc(b.key, b.value),
                _ => continue,
            }
        }
        mem::replace(&mut *table, temp.table.into_inner());
    }


    // Compiler warns about a field not exported in core.
    #[allow(deprecated)]
    fn get_index(&self, key: &K) -> usize {
        let mut hasher = SipHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.table.read().len()
    }


    pub fn remove(&mut self, key: K) {
        let index = self.get_index(&key);
        let mut table = self.table.write();
        let (begin, end) = table.split_at_mut(index);
        for entry in end.iter_mut().chain(begin) {
            match entry {
                &mut HashMapMember::Tombstone => continue,
                &mut HashMapMember::None => return,
                &mut HashMapMember::Bucket(ref mut b) if b.key != key => continue,
                loc @ &mut HashMapMember::Bucket(_) => {
                    *loc = HashMapMember::Tombstone;
                    return;
                }
            }
        }
    }


    pub fn insert_rc(&mut self, key: K, value: Arc<V>) {
        self.count += 1;
        if self.should_resize() {
            self.rebalance();
        }
        let index = self.get_index(&key);
        let mut table = self.table.write();
        let (begin, end) = table.split_at_mut(index);
        for entry in end.iter_mut().chain(begin) {
            match entry {
                &mut HashMapMember::Tombstone => continue,
                loc @ &mut HashMapMember::None => {
                    *loc = HashMapMember::Bucket(Bucket::new(key, value));
                    return;
                }
                &mut HashMapMember::Bucket(ref mut b) => {
                    if b.key == key {
                        b.value = value;
                        return;
                    } else {
                        continue;
                    }
                }
            }
        }
    }


    pub fn insert(&mut self, key: K, value: V) {
        self.insert_rc(key, Arc::new(value))
    }


    pub fn contains(&self, key: &K) -> bool {
        let index = self.get_index(key);
        let table = self.table.read();
        let (begin, end) = table.split_at(index);
        for entry in end.iter().chain(begin) {
            match *entry {
                HashMapMember::Tombstone => continue,
                HashMapMember::None => return false,
                HashMapMember::Bucket(ref b) => {
                    if &b.key == key {
                        return true;
                    } else {
                        continue;
                    }
                }
            }
        }
        false
    }


    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        let index = self.get_index(key);
        let mut table = self.table.write();
        let (begin, end) = table.split_at_mut(index);
        for entry in end.iter_mut().chain(begin) {
            match *entry {
                HashMapMember::Tombstone => continue,
                HashMapMember::None => return None,
                HashMapMember::Bucket(ref b) => {
                    if &b.key == key {
                        return Some(b.value.clone());
                    } else {
                        continue;
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use alloc::arc::Arc;
    use super::HashMap;


    #[test]
    fn test_new() {
        let ht = HashMap::<i32, i32>::new(10);
        let length = ht.table.read().len();
        assert_eq!(length, 10);

        let ht1 = HashMap::<&str, &str>::new(100);
        let length = ht1.table.read().len();
        assert_eq!(length, 100);

        #[allow(unused)]
        struct Custom<'a> {
            int_field: i32,
            str_field: &'a str,
        }

        let ht2 = HashMap::<i32, Custom>::new(1000);
        let length = ht2.table.read().len();
        assert_eq!(length, 1000);
    }


    #[test]
    fn test_basic_operations() {
        let mut ht = HashMap::<i32, i32>::new(10);
        let k = 42;
        ht.insert(k, 43);
        assert!(ht.contains(&k));
        assert_eq!(ht.get(&k), Some(Arc::new(43)));

        ht.remove(42);
        assert!(!ht.contains(&k));
        assert_eq!(ht.get(&k), None);

        ht.insert(42, 44);
        assert!(ht.contains(&k));
        assert_eq!(ht.get(&k), Some(Arc::new(44)));

        ht.insert(42, 45);
        assert!(ht.contains(&k));
        assert_eq!(ht.get(&k), Some(Arc::new(45)));
    }

    #[test]
    fn test_rebalance() {

        let initial_size = 4;
        let mut ht = HashMap::<usize, usize>::new(initial_size);
        assert_eq!(ht.count, 0);

        ht.insert(0, 0);
        assert_eq!(ht.count, 1);
        assert_eq!(ht.table.read().len(), 4);

        ht.insert(1, 1);
        assert_eq!(ht.count, 2);
        assert_eq!(ht.table.read().len(), 4);

        ht.insert(2, 2);
        assert_eq!(ht.count, 3);
        assert_eq!(ht.table.read().len(), 8);

        ht.insert(3, 3);
        assert_eq!(ht.count, 4);
        assert!(!ht.should_resize());
        assert_eq!(ht.table.read().len(), 8);

        ht.insert(4, 4);
        assert_eq!(ht.count, 5);
        assert!(!ht.should_resize());
        assert_eq!(ht.table.read().len(), 8);

        ht.insert(5, 5);
        assert_eq!(ht.count, 6);
        assert!(!ht.should_resize());
        assert_eq!(ht.table.read().len(), 16);
    }
}
