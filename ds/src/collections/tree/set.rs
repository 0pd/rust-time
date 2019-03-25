use std::marker::PhantomData;
use super::map::TreeMap;

pub struct TreeSet<T> {
    map: TreeMap<T, ()>
}

pub struct Iter<T> {
    _d: PhantomData<T>
}

impl<T: Ord> TreeSet<T> {
    fn new() -> TreeSet<T> {
        TreeSet {
            map: TreeMap::new()
        }
    }

    pub fn clear(&mut self) {
        self.map.clear()
    }

    pub fn contains(&self, value: &T) -> bool {
        self.map.contains_key(value)
    }

    pub fn get(&self, value: &T) -> Option<&T> {
        unimplemented!()
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.map.insert(value, ()).is_some()
    }

    pub fn replace(&mut self, value: T) -> Option<T> {
        unimplemented!()
    }

    pub fn remove(&mut self, value: &T) -> bool {
        self.map.remove(value).is_some()
    }

    pub fn take(&mut self, value: &T) -> Option<T> {
        unimplemented!()
    }
}

impl<T> TreeSet<T> {
    pub fn iter(&self) -> Iter<T> {
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

#[cfg(test)]
mod tests {

}
