use std::marker::PhantomData;

pub struct TreeMap<K, V> {
    _k: PhantomData<K>,
    _v: PhantomData<V>,
    length: usize,
}

pub struct Entry<K, V> {
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

pub struct Iter<K, V> {
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

pub struct Keys<K, V> {
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

pub struct Values<K, V> {
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

impl<K: Ord, V> TreeMap<K, V> {
    pub fn new() -> TreeMap<K, V> {
        unimplemented!()
    }

    pub fn clear(&mut self) {
        unimplemented!()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        unimplemented!()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        unimplemented!()
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        unimplemented!()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        unimplemented!()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
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
