use alloc::vec::Vec;
use crate::map::node::Node;

pub(super) struct Pool<K, V> {
    pub(super) buffer: Vec<Node<K, V>>,
    pub(super) unused: Vec<u32>
}

impl<K: Copy + Default, V: Clone + Default> Pool<K, V> {

    #[inline]
    pub(super) fn new(capacity: usize) -> Self {
        let capacity = capacity.max(8);
        let mut store = Self {
            buffer: Vec::with_capacity(capacity),
            unused: Vec::with_capacity(capacity),
        };
        store.reserve(capacity);
        store
    }

    #[inline]
    fn reserve(&mut self, length: usize) {
        debug_assert!(length > 0);
        let n = self.buffer.len() as u32;
        let l = length as u32;
        self.buffer.reserve(length);
        self.buffer.resize(self.buffer.len() + length, Node::default());
        self.unused.reserve(length);
        self.unused.extend((n..n + l).rev());
    }

    #[inline]
    pub(super) fn get_free_index(&mut self) -> u32 {
        if self.unused.is_empty() {
            self.reserve(self.unused.capacity());
        }
        self.unused.pop().unwrap()
    }


    #[inline(always)]
    pub(super) fn put_back(&mut self, index: u32) {
        self.unused.push(index)
    }
}