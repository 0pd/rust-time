use std::marker::PhantomData;
use std::borrow::Borrow;

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

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
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
