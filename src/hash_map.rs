use alloc::rc::Rc;
use spin::RwLock;
use core::hash::{Hash, Hasher};
// Compiler warns about a field not exported in core.
#[allow(deprecated)]
use core::hash::SipHasher;
use collections::vec::Vec;
use core::cmp;

enum HashMapMember<K: Hash + cmp::PartialEq, V> {
    None,
    Tombstone,
    Bucket(Bucket<K, V>)
}

struct Bucket<K: Hash + cmp::PartialEq, V> {
    key: K,
    value: Rc<V>,
}

pub struct HashMap<K: Hash + cmp::PartialEq, V> {
    count: usize,
    table: RwLock<Vec<HashMapMember<K, V>>>
}

impl<K: Hash + cmp::PartialEq, V> Bucket<K, V> {
    fn new(key: K, value: V) -> Self {
        Bucket{ key: key, value: Rc::new(value) }
    }
}

impl<K: Hash + cmp::PartialEq, V> HashMap<K, V> {

    pub fn new(size: usize) -> Self {
        let mut table = vec![];
        for _ in 0..size {
            table.push(HashMapMember::None);
        }
        HashMap{ count: 0, table: RwLock::new(table) }
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

        for i in index..self.table.write().len() {
            match self.table.write()[i] {
                HashMapMember::Tombstone => continue,
                HashMapMember::None => return,
                HashMapMember::Bucket(ref mut b) => {
                    if b.key == key {
                        self.table.write()[i] = HashMapMember::Tombstone;
                        return;
                    } else {
                        continue;
                    }
                }
            }
        }

        for i in 0..index {
            match self.table.write()[i] {
                HashMapMember::Tombstone => continue,
                HashMapMember::None => return,
                HashMapMember::Bucket(ref mut b) => {
                    if b.key == key {
                        self.table.write()[i] = HashMapMember::Tombstone;
                        return;
                    } else {
                        continue;
                    }
                }
            }
        }
    }


    pub fn insert(&mut self, key: K, value: V) {
        let index = self.get_index(&key);
        extern crate std;

        for i in index..self.table.write().len() {
            match self.table.write()[i] {
                HashMapMember::Tombstone => continue,
                HashMapMember::None => {
                    self.table.write()[i] = HashMapMember::Bucket(Bucket::new(key, value));
                    return;
                }
                HashMapMember::Bucket(ref mut b) => {
                    if b.key == key {
                        b.value = Rc::new(value);
                        self.count += 1;
                        return;
                    } else {
                        continue;
                    }
                }
            }
        }

        for i in 0..index {
            match self.table.write()[i] {
                HashMapMember::Tombstone => continue,
                HashMapMember::None => {
                    self.table.write()[i] = HashMapMember::Bucket(Bucket::new(key, value));
                    return;
                }
                HashMapMember::Bucket(ref mut b) => {
                    if b.key == key {
                        b.value = Rc::new(value);
                        self.count += 1;
                        return;
                    } else {
                        continue;
                    }
                }
            }
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        let index = self.get_index(key);

        for i in index..self.table.read().len() {
            match self.table.read()[i] {
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

        for i in index..self.table.read().len() {
            match self.table.read()[i] {
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


    pub fn get(&self, key: &K) -> Option<Rc<V>> {
        let index = self.get_index(key);

        for i in index..self.table.read().len() {
            match self.table.read()[i] {
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

        for i in index..self.table.read().len() {
            match self.table.read()[i] {
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
    use alloc::rc::Rc;
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
        ht.insert(42, 43);
        /*
        let k = 42;
        assert!(ht.contains(&k));
        assert_eq!(ht.get(&k), Some(Rc::new(43)));

        ht.remove(42);
        assert!(!ht.contains(&k));
        assert_eq!(ht.get(&k), None);

        ht.insert(42, 44);
        assert!(ht.contains(&k));
        assert_eq!(ht.get(&k), Some(Rc::new(44)));

        ht.insert(42, 45);
        assert!(ht.contains(&k));
        assert_eq!(ht.get(&k), Some(Rc::new(45)));
        */
    }
}
