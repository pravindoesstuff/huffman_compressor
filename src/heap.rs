// Implemented as a MaxHeap
#[derive(Debug)]
pub struct Heap<T> {
    heap: Vec<T>,
}

impl<T: std::cmp::PartialOrd> Heap<T> {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
    pub fn len(&self) -> usize {
        self.heap.len()
    }
    pub fn peek(&self) -> Option<&T> {
        self.heap.first()
    }
    fn left_child(index: usize) -> usize {
        2 * index + 1
    }
    fn right_child(index: usize) -> usize {
        2 * index
    }
    fn parent(index: usize) -> usize {
        index / 2
    }
    pub fn insert(&mut self, value: T) {
        self.heap.push(value);
        self.bubble_up(self.heap.len() - 1)
    }
    fn bubble_up(&mut self, index: usize) {
        if index == 0 {
            return;
        }
        let parent_index = Self::parent(index);
        if self.heap[index] < self.heap[parent_index] {
            self.heap.swap(index, parent_index);
            self.bubble_up(parent_index);
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }
        let returned_item = self.heap.swap_remove(0);
        self.sift_down(0);
        Some(returned_item)
    }
    fn sift_down(&mut self, index: usize) {
        let largest_child = self.cmp_children(index);
        if index == largest_child {
            return;
        }
        self.heap.swap(index, largest_child);
        self.sift_down(largest_child);
    }

    fn cmp_children(&self, index: usize) -> usize {
        let right_child = Self::right_child(index);
        if right_child >= self.heap.len() {
            return index;
        }
        let left_child = Self::left_child(index);
        if left_child >= self.heap.len() {
            return if self.heap[right_child] < self.heap[index] {
                right_child
            } else {
                index
            };
        }
        if self.heap[left_child] < self.heap[right_child] {
            left_child
        } else {
            right_child
        }
    }
}
