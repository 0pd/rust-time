use std::marker::PhantomData;
use super::map::TreeMap;
use std::iter::FromIterator;
use std::borrow::Borrow;

pub struct TreeSet<T> {
    map: TreeMap<T, ()>
}

pub struct Iter<'a, T> {
    _d: PhantomData<&'a T>
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

    pub fn contains<Q>(&self, value: &Q) -> bool
        where T: Borrow<Q>,
              Q: Ord + ?Sized {
        self.map.contains_key(value)
    }

    pub fn get<Q>(&self, value: &Q) -> Option<&T>
        where T: Borrow<Q>,
              Q: Ord + ?Sized {
        unimplemented!()
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.map.insert(value, ()).is_some()
    }

    pub fn replace(&mut self, value: T) -> Option<T> {
        unimplemented!()
    }

    pub fn remove<Q>(&mut self, value: &Q) -> bool
        where T: Borrow<Q>,
              Q: Ord + ?Sized {
        self.map.remove(value).is_some()
    }

    pub fn take<Q>(&mut self, value: &Q) -> Option<T>
        where T: Borrow<Q>,
              Q: Ord + ?Sized {
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
        self.len() == 0
    }
}

impl<T: Ord> FromIterator<T> for TreeSet<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> TreeSet<T> {
        unimplemented!()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::TreeSet;

    #[test]
    fn clear() {
        let mut v = TreeSet::new();
        v.insert(1);
        v.clear();
        assert!(v.is_empty());
    }

    #[test]
    fn contains() {
        let set: TreeSet<_> = [1, 2, 3].iter().cloned().collect();
        assert_eq!(set.contains(&1), true);
        assert_eq!(set.contains(&4), false);
    }

    #[test]
    fn get() {
        let set: TreeSet<_> = [1, 2, 3].iter().cloned().collect();
        assert_eq!(set.get(&2), Some(&2));
        assert_eq!(set.get(&4), None);
    }

    #[test]
    fn insert() {
        let mut set = TreeSet::new();

        assert_eq!(set.insert(2), true);
        assert_eq!(set.insert(2), false);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn replace() {
        let mut set = TreeSet::new();
        set.insert(Vec::<i32>::new());

        assert_eq!(set.get(&[][..]).unwrap().capacity(), 0);
        set.replace(Vec::with_capacity(10));
        assert_eq!(set.get(&[][..]).unwrap().capacity(), 10);
    }

    #[test]
    fn remove() {
        let mut set = TreeSet::new();

        set.insert(2);
        assert_eq!(set.remove(&2), true);
        assert_eq!(set.remove(&2), false);
    }

    #[test]
    fn take() {
        let mut set: TreeSet<_> = [1, 2, 3].iter().cloned().collect();
        assert_eq!(set.take(&2), Some(2));
        assert_eq!(set.take(&2), None);
    }

    #[test]
    fn iter() {
        let set: TreeSet<usize> = [3, 1, 2].iter().cloned().collect();
        let mut set_iter = set.iter();
        assert_eq!(set_iter.next(), Some(&1));
        assert_eq!(set_iter.next(), Some(&2));
        assert_eq!(set_iter.next(), Some(&3));
        assert_eq!(set_iter.next(), None);
    }

    #[test]
    fn len() {
        let mut v = TreeSet::new();
        assert_eq!(v.len(), 0);
        v.insert(1);
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn is_empty() {
        let mut v = TreeSet::new();
        assert!(v.is_empty());
        v.insert(1);
        assert!(!v.is_empty());
    }
}
