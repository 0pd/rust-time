use std::marker::PhantomData;
use std::borrow::Borrow;
use std::ops::Index;
use std::iter::FromIterator;

pub struct TreeMap<K, V> {
    _k: PhantomData<K>,
    _v: PhantomData<V>,
    length: usize,
}

pub struct Entry<'a, K, V> {
    _k: PhantomData<&'a K>,
    _v: PhantomData<V>,
}

pub struct Iter<'a, K, V> {
    _k: PhantomData<&'a K>,
    _v: PhantomData<V>,
}

pub struct Keys<'a, K, V> {
    _k: PhantomData<&'a K>,
    _v: PhantomData<V>,
}

pub struct Values<'a, K, V> {
    _k: PhantomData<K>,
    _v: PhantomData<&'a V>,
}

impl<K: Ord, V> TreeMap<K, V> {
    pub fn new() -> TreeMap<K, V> {
        unimplemented!()
    }

    pub fn clear(&mut self) {
        unimplemented!()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where K: Borrow<Q>,
              Q: Ord + ?Sized {
        unimplemented!()
    }

    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
        where K: Borrow<Q>,
              Q: Ord + ?Sized {
        unimplemented!()
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
        where K: Borrow<Q>,
              Q: Ord + ?Sized {
        unimplemented!()
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where K: Borrow<Q>,
              Q: Ord + ?Sized {
        unimplemented!()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        unimplemented!()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
        where K: Borrow<Q>,
              Q: Ord + ?Sized {
        unimplemented!()
    }

    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        unimplemented!()
    }
}

impl<K, V> TreeMap<K, V> {
    pub fn iter(&self) -> Iter<K, V> {
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn keys(&self) -> Keys<K, V> {
        unimplemented!()
    }

    pub fn values(&self) -> Values<K, V> {
        unimplemented!()
    }
}

impl<K, V> FromIterator<(K, V)> for TreeMap<K, V>
    where K: Ord {
    fn from_iter<I: IntoIterator<Item=(K, V)>>(iter: I) -> Self {
        unimplemented!()
    }
}

impl<'a, K, V, Q> Index<&'a Q> for TreeMap<K, V>
    where K: Ord + Borrow<Q>,
          Q: Ord + ?Sized {
    type Output = V;

    fn index(&self, index: &Q) -> &V {
        unimplemented!()
    }
}

impl<'a, K, V> Entry<'a, K, V>
    where K: Ord {
    pub fn or_insert(&self, default: V) -> &'a mut V {
        unimplemented!()
    }

    pub fn key(&self) -> &K {
        unimplemented!()
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
    where K: 'a,
          V: 'a {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        unimplemented!()
    }
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<&'a K> {
        unimplemented!()
    }
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<&'a V> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::TreeMap;

    #[test]
    fn clear() {
        let mut a = TreeMap::new();
        a.insert(1, "a");
        a.clear();
        assert!(a.is_empty());
    }

    #[test]
    fn get() {
        let mut map = TreeMap::new();
        map.insert(1, "a");
        assert_eq!(map.get(&1), Some(&"a"));
        assert_eq!(map.get(&2), None);
    }

    #[test]
    fn get_key_value() {
        let mut map = TreeMap::new();
        map.insert(1, "a");
        assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
        assert_eq!(map.get_key_value(&2), None);
    }

    #[test]
    fn contains_key() {
        let mut map = TreeMap::new();
        map.insert(1, "a");
        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&2), false);
    }

    #[test]
    fn insert() {
        let mut map = TreeMap::new();
        assert_eq!(map.insert(37, "a"), None);
        assert_eq!(map.is_empty(), false);

        map.insert(37, "b");
        assert_eq!(map.insert(37, "c"), Some("b"));
        assert_eq!(map[&37], "c");
    }

    #[test]
    fn remove() {
        let mut map = TreeMap::new();
        map.insert(1, "a");
        assert_eq!(map.remove(&1), Some("a"));
        assert_eq!(map.remove(&1), None);
    }

    #[test]
    fn entry() {
        let mut count: TreeMap<&str, usize> = TreeMap::new();

        // count the number of occurrences of letters in the vec
        for x in vec!["a", "b", "a", "c", "a", "b"] {
            *count.entry(x).or_insert(0) += 1;
        }

        assert_eq!(count["a"], 3);
    }

    #[test]
    fn iter() {
        let mut map = TreeMap::new();
        map.insert(3, "c");
        map.insert(2, "b");
        map.insert(1, "a");

        for (key, value) in map.iter() {
            println!("{}: {}", key, value);
        }

        let (first_key, first_value) = map.iter().next().unwrap();
        assert_eq!((*first_key, *first_value), (1, "a"));
    }

    #[test]
    fn keys() {
        let mut a = TreeMap::new();
        a.insert(2, "b");
        a.insert(1, "a");

        let keys: Vec<_> = a.keys().cloned().collect();
        assert_eq!(keys, [1, 2]);
    }

    #[test]
    fn values() {
        let mut a = TreeMap::new();
        a.insert(1, "hello");
        a.insert(2, "goodbye");

        let values: Vec<&str> = a.values().cloned().collect();
        assert_eq!(values, ["hello", "goodbye"]);
    }

    #[test]
    fn len() {
        let mut a = TreeMap::new();
        assert_eq!(a.len(), 0);
        a.insert(1, "a");
        assert_eq!(a.len(), 1);
    }

    #[test]
    fn is_empty() {
        let mut a = TreeMap::new();
        assert!(a.is_empty());
        a.insert(1, "a");
        assert!(!a.is_empty());
    }
}
