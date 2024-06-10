type Entry<K, V> = Vec<(K, V)>;

pub struct HashMap<K, V> where
    K: PartialEq + Clone,
    V: Clone, {
      hash_fn: Box<dyn (Fn(&K) -> usize)>,
      store: Box<[Entry<K, V>]>,
      pub length: usize,
}

impl HashMap<K, V> {

    // TODO: implement constructor/new!

    pub fn insert(&mut self, key: K, value: V) {
       let h = (self.hash_fn)(&key);
       let idx = h & (self.store.len() - 1);
       match self.store[idx].iter().position(|e| e.0 == key) {
            Some(pos) => self.store[idx][pos] = (key, value),
            None => {
                self.store[idx].push((key, value));
                self.length += 1
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let h = (self.hash_fn)(key);
        let idx = h & (self.store.len() - 1);
        self.store[idx]
            .iter()
            .find(|e| e.0 == *key)
            .map(|e| e.1.clone())
         }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let h = (self.hash_fn)(&key);
        let idx = h & (self.store.len() - 1);
        match self.store[idx].iter().position(|e| e.0 == key) {
            Some(pos) => {
                self.length -= 1;
                Some(self.store[idx].remove(pos).1)
                }
            _ => None,
        }
    }
}


pub type LocationCache = HashMap<String, LocationInformation>;
