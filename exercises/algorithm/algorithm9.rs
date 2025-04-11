/*
	heap
	This question requires you to implement a binary heap function
*/
use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            //items: vec![T::default()],
            items: vec![],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);
        self.count += 1;

        self.swim(self.count-1);    
    }

    fn swim(&mut self, idx:usize) {
        let mut curr_idx = idx;
        let mut parent_idx = self.parent_idx(curr_idx);
        while curr_idx> 0 && (self.comparator)(&self.items[curr_idx], &self.items[parent_idx]) {
            self.items.swap(curr_idx, parent_idx);
            curr_idx = parent_idx;
            parent_idx = self.parent_idx(curr_idx);
        }
    }

    fn sink(&mut self, idx:usize) {
        let mut curr_idx = idx;
        let mut child_to_switch = self.smallest_child_idx(curr_idx);
        while child_to_switch < self.count && !(self.comparator)(&self.items[curr_idx], &self.items[child_to_switch]){
            self.items.swap(curr_idx, child_to_switch);
            curr_idx = child_to_switch;
            child_to_switch = self.smallest_child_idx(curr_idx);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        //self.left_child_idx(idx) <= self.count
        self.left_child_idx(idx) < self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        //idx * 2
        idx*2 + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    //used for sinking
    //actually get the next target child accroding to comparator
    //if it's MaxHeap, then get the bigger value child
    //if it's MinHeap, then get the smaller value child
    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        if self.count <= self.right_child_idx(idx) {
            return self.left_child_idx(idx);
        }

        let (left_child_idx, right_child_idx) = (self.left_child_idx(idx), self.right_child_idx(idx));
        if (self.comparator)(&self.items[left_child_idx], &self.items[right_child_idx]) {
            left_child_idx
        } else {
            right_child_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    //pop from root, change self.items
    fn next(&mut self) -> Option<T> {
        //TODO
        if self.count == 0 {
            return None
        }

        let top = self.items.swap_remove(0);
        self.count -=1;
        self.sink(0);
        Some(top)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}