use std::marker::PhantomData;
use std::borrow::Borrow;
use std::ops::Index;
use std::iter::FromIterator;
use self::node::*;

pub struct TreeMap<K, V> {
    root: RootNode<K, V>,
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
        TreeMap {
            root: RootNode::new(),
            length: 0,
        }
    }

    pub fn clear(&mut self) {
        self.root = RootNode::new();
        self.length = 0;
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where K: Borrow<Q>,
              Q: Ord + ?Sized {
        self.get_key_value(key).map(|(k, v)| v)
    }

    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
        where K: Borrow<Q>,
              Q: Ord + ?Sized {
        self.root.search(key)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
        where K: Borrow<Q>,
              Q: Ord + ?Sized {
        self.get(key).is_some()
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where K: Borrow<Q>,
              Q: Ord + ?Sized {
        unimplemented!()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let old_value = self.root.insert(key, value);
        if old_value.is_none() {
            self.length += 1;
        }

        old_value
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
        where K: Borrow<Q>,
              Q: Ord + ?Sized {
        let old_value = self.root.remove(key);
        if old_value.is_some() {
            self.length -= 1;
        }

        old_value
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
    fn from_iter<I: IntoIterator<Item=(K, V)>>(iter: I) -> TreeMap<K, V> {
        let mut map = TreeMap::new();
        for (key, value) in iter {
            map.insert(key, value);
        }

        map
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

mod node {
    use std::rc::{Weak, Rc};
    use std::borrow::Borrow;
    use std::cmp::Ordering;

    pub struct RootNode<K, V> {
        node: Option<Rc<Node<K, V>>>
    }

    impl<K, V> RootNode<K, V> {
        pub fn new() -> RootNode<K, V> {
            RootNode {
                node: None
            }
        }

        pub fn search<Q>(&self, key: &Q) -> Option<(&K, &V)>
            where K: Borrow<Q>,
                  Q: Ord + ?Sized {
            if let Some(node) = self.node.borrow() {
                node.search(key)
            } else {
                None
            }
        }

        pub fn insert(&mut self, key: K, value: V) -> Option<V> {
            if self.node.is_none() {
                self.node = Some(Rc::new(Node::Leaf(LeafNode {
                    key,
                    value,
                    parent: Weak::default(),
                })));

                None
            } else {
                unimplemented!()
            }
        }

        pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
            where K: Borrow<Q>,
                  Q: Ord + ?Sized {
            if self.node.is_none() {
                None
            } else {
                unimplemented!()
            }
        }
    }

    enum SearchResult<'a, K, V> {
        Found(Option<(&'a K, &'a V)>),
        GoDown(Rc<Node<K, V>>)
    }

    fn search<K, V, Q>(node: Node<K, V>, key: &Q) -> Option<(&K, &V)>
        where K: Borrow<Q>,
              Q: Ord + ?Sized {
        let mut current = Rc::new(node);
        loop {
            match search_internal(current, key) {
                SearchResult::Found(result) => {
                    return result;
                }
                SearchResult::GoDown(node) => {
                    current = node;
                }
            }
        }
    }

    fn search_internal<K, V, Q>(node: Rc<Node<K, V>>, key: &Q) -> SearchResult<K, V>
        where K: Borrow<Q>,
              Q: Ord + ?Sized {
        match node.clone().borrow() {
            Node::Leaf(leaf) => {
                let result = if let Ordering::Equal = key.cmp(leaf.key.borrow()) {
                    Some((&leaf.key, &leaf.value))
                } else {
                    None
                };
                SearchResult::Found(result)
            }
            Node::TwoNode(two) => {
                if let Ordering::Less = key.cmp((*two.right_min).borrow()) {
                    SearchResult::GoDown(two.left_child.clone())
                } else {
                    SearchResult::GoDown(two.right_child.clone())
                }
            }
            Node::ThreeNode(three) => {
                if let Ordering::Less = key.cmp((*three.right_min).borrow()) {
                    if let Ordering::Less = key.cmp((*three.right_min).borrow()) {
                        SearchResult::GoDown(three.left_child.clone())
                    } else {
                        SearchResult::GoDown(three.middle_child.clone())
                    }
                } else {
                    SearchResult::GoDown(three.right_child.clone())
                }
            }
        }
    }

    trait NodeT<K, V> {
        fn search<Q>(&self, key: &Q) -> Option<(&K, &V)>
            where K: Borrow<Q>,
                  Q: Ord + ?Sized;
    }

    enum Node<K, V> {
        Leaf(LeafNode<K, V>),
        TwoNode(TwoNode<K, V>),
        ThreeNode(ThreeNode<K, V>),
    }

    impl<K, V> NodeT<K, V> for Node<K, V> {
        fn search<Q>(&self, key: &Q) -> Option<(&K, &V)>
            where K: Borrow<Q>,
                  Q: Ord + ?Sized {
            match self {
                Node::Leaf(leaf) => {
                    leaf.search(key)
                }
                Node::TwoNode(two) => {
                    two.search(key)
                }
                Node::ThreeNode(three) => {
                    three.search(key)
                }
            }
        }
    }

    struct LeafNode<K, V> {
        key: K,
        value: V,
        parent: Weak<Node<K, V>>,
    }

    impl<K, V> NodeT<K, V> for LeafNode<K, V> {
        fn search<Q>(&self, key: &Q) -> Option<(&K, &V)>
            where K: Borrow<Q>,
                  Q: Ord + ?Sized {
            if let Ordering::Equal = key.cmp(self.key.borrow()) {
                Some((&self.key, &self.value))
            } else {
                None
            }
        }
    }

    struct TwoNode<K, V> {
        right_min: Rc<K>,
        left_child: Rc<Node<K, V>>,
        right_child: Rc<Node<K, V>>,
        parent: Weak<Node<K, V>>,
    }

    impl<K, V> NodeT<K, V> for TwoNode<K, V> {
        fn search<Q>(&self, key: &Q) -> Option<(&K, &V)>
            where K: Borrow<Q>,
                  Q: Ord + ?Sized {
            if let Ordering::Less = key.cmp((*self.right_min).borrow()) {
                self.left_child.search(key)
            } else {
                self.right_child.search(key)
            }
        }
    }

    struct ThreeNode<K, V> {
        middle_min: Rc<K>,
        right_min: Rc<K>,
        left_child: Rc<Node<K, V>>,
        middle_child: Rc<Node<K, V>>,
        right_child: Rc<Node<K, V>>,
        parent: Weak<Node<K, V>>,
    }

    impl<K, V> NodeT<K, V> for ThreeNode<K, V> {
        fn search<Q>(&self, key: &Q) -> Option<(&K, &V)>
            where K: Borrow<Q>,
                  Q: Ord + ?Sized {
            if let Ordering::Less = key.cmp((*self.right_min).borrow()) {
                if let Ordering::Less = key.cmp((*self.right_min).borrow()) {
                    self.left_child.search(key)
                } else {
                    self.middle_child.search(key)
                }
            } else {
                self.right_child.search(key)
            }
        }
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
